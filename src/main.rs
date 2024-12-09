//#![forbid(missing_docs)]
use core::cell::UnsafeCell;
use std::path::PathBuf;

use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer};
use registry::TypeRegistry;
use rfd::AsyncFileDialog;
use tokio::sync::oneshot;
use views::assembly::AssemblyTab;
use views::configure::{ImportState, ImportWindow};
use views::console::ConsoleTab;
use views::functions::FunctionsTab;

pub mod error;
pub mod format;
pub mod registry;
pub mod views;

// TODO: Global `Style`s for text, add CFA to populate, render the loaded file to the assembly view

// flag this as tokio::main so we can use tokio::spawn inside update()
#[tokio::main]
async fn main() -> eframe::Result {
    // Set a default window size
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 720.0]),
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        "Ferrox Disassembler",
        native_options,
        Box::new(|_| Ok(Box::new(FerroxApplication::new()))),
    )
}

// TODO: make file selector its own view?
#[derive(Default, PartialEq)]
enum DialogState {
    #[default]
    Idle,
    Selecting,
    Loaded,
    Cancelled,
}

// Current state of the Ferrox Application
#[derive(Default, PartialEq)]
enum FerroxState {
    #[default]
    Init,
    Configure,
    Analyzing,
    Interactable,
}

// All supported File Types
#[derive(Default, PartialEq, Copy, Clone)]
pub enum BinaryFormat {
    BinaryFile,
    #[default]
    GameCubeDOL,
}

// All supported Architectures
#[derive(Default, PartialEq, Copy, Clone)]
pub enum ProcessorType {
    #[default]
    PowerPCGekko,
}

// Main Ferrox Application, responsible for managing all disassembler state.
struct FerroxApplication {
    // File Selector
    dialog_state: DialogState,
    dialog_info: Option<oneshot::Receiver<Option<(PathBuf, Vec<u8>)>>>,
    loaded_file: (PathBuf, Vec<u8>),
    loaded_state: FerroxState,
    style: Option<Style>,

    // Import Menu State
    binary_format: BinaryFormat,
    processor_type: ProcessorType,
    import_window_open: bool,
    import: ImportWindow,

    // Assembly View
    tree: UnsafeCell<DockState<String>>,
    _registry: TypeRegistry,
    assembly: AssemblyTab,
    functions: FunctionsTab,
    console: ConsoleTab,
}

impl FerroxApplication {
    fn new() -> Self {
        // Initial Main Window Tabs
        let mut dock_state = DockState::new(vec![
            "Ferrox View-A".to_owned(),
            "Hex-View 1".to_owned(),
            "Local Types".to_owned(),
            "Imports".to_owned(),
            "Exports".to_owned(),
        ]);
        // Rename "Eject"
        "Undock".clone_into(&mut dock_state.translations.tab_context_menu.eject_button);
        // Initial Side Tab(s)
        dock_state.main_surface_mut().split_left(NodeIndex::root(), 0.2, vec!["Functions".to_owned()]);
        // Initial Bottom Tab(s)
        dock_state.main_surface_mut().split_below(NodeIndex::root(), 0.8, vec!["Output".to_owned()]);

        // Default State
        Self {
            dialog_state: DialogState::Idle,
            dialog_info: None,
            loaded_file: (PathBuf::new(), Vec::new()),
            loaded_state: FerroxState::default(),
            style: None,

            binary_format: BinaryFormat::default(),
            processor_type: ProcessorType::default(),
            import_window_open: false,
            import: ImportWindow::new(
                vec![
                    ("GameCube Binary (DOL)", BinaryFormat::GameCubeDOL),
                    ("Binary File", BinaryFormat::BinaryFile),
                ],
                vec![("PowerPC Gekko/Broadway (Big Endian)", ProcessorType::PowerPCGekko)],
            ),

            tree: dock_state.into(),
            _registry: TypeRegistry::new(),
            assembly: AssemblyTab {},
            functions: FunctionsTab {},
            console: ConsoleTab {},
        }
    }
}

// Support Trait for Docking Layout
impl TabViewer for FerroxApplication {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.as_str().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.as_str() {
            "Ferrox View-A" => self.assembly.update(ui),
            "Functions" => self.functions.update(ui),
            "Output" => self.console.update(ui),
            _ => {
                ui.label(tab.as_str());
            }
        };
    }

    // TODO: closeable, context_menu, on_close
}

// Main egui Rendering State
impl eframe::App for FerroxApplication {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // TODO: make menu_bar its own view? This will get way more options over time
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        if self.dialog_state != DialogState::Selecting {
                            // Create a new channel to receive file data once we've loaded it
                            let (tx, rx) = oneshot::channel();
                            self.dialog_info = Some(rx);
                            let ctx = ctx.clone();

                            // Spawn a new task to wait on the user.
                            tokio::spawn(async move {
                                // Spawn a new window to open a file
                                let result = AsyncFileDialog::new()
                                    .add_filter("GameCube Binary", &["dol"])
                                    // TODO: add saving to a database
                                    .add_filter("Ferrox Database", &["frx"])
                                    .add_filter("Any file", &["*"])
                                    .set_directory(std::env::current_dir().ok().unwrap())
                                    .pick_file()
                                    .await;

                                // Check if we've opened a file and try to read its data
                                let result = match result {
                                    Some(file_path) => {
                                        Some((file_path.path().to_path_buf(), file_path.read().await))
                                    }
                                    None => None,
                                };

                                // Respond to the main thread and tell it to update
                                let _ = tx.send(result);
                                ctx.request_repaint();
                            });
                        }
                    }
                });
            })
        });

        // Waiting state for file selector
        if let Some(receiver) = &mut self.dialog_info {
            match receiver.try_recv() {
                Err(oneshot::error::TryRecvError::Closed) => self.dialog_state = DialogState::Cancelled,
                // If it's empty, we haven't yet received any signal
                Err(oneshot::error::TryRecvError::Empty) => (),
                // We've actually gotten a response, process it.
                Ok(Some(file_info)) => {
                    self.dialog_state = DialogState::Loaded;
                    self.loaded_file = file_info;
                    self.loaded_state = FerroxState::Configure;
                    self.import_window_open = true;
                }
                Ok(None) => self.dialog_state = DialogState::Cancelled,
            }
        }

        match self.loaded_state {
            // Initial state (when a file isn't opened)
            FerroxState::Init => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Welcome to Ferrox!");
                    ui.label("Select a file to get started.")
                });
            }
            // Configuring settings for the file we just opened
            FerroxState::Configure => {
                // Continue to show this until we actual load in the file
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Welcome to Ferrox!");
                    ui.label("Select a file to get started.");
                });

                let mut close_window = false;
                egui::Window::new("Configure New Import")
                    .collapsible(false)
                    .open(&mut self.import_window_open)
                    .show(ctx, |ui| {
                        match self.import.update(ui, &mut self.binary_format, &mut self.processor_type) {
                            ImportState::Waiting => (),
                            ImportState::Configured => {
                                self.loaded_state = FerroxState::Analyzing;
                                close_window = true;
                            }
                            ImportState::Cancelled => {
                                self.loaded_state = FerroxState::Init;
                                close_window = true;
                            }
                        }
                    });

                if close_window {
                    self.import_window_open = false;
                }
            }
            // Analyze the binary before we display it in the main window
            FerroxState::Analyzing => {
                // TODO: CFA
                self.loaded_state = FerroxState::Interactable;
            }
            // Main state where the user can begin using the disassembly
            FerroxState::Interactable => {
                egui::CentralPanel::default()
                    .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.))
                    .show(ctx, |ui| {
                        let style = match &self.style {
                            Some(style) => style.clone(),
                            None => {
                                let mut style = Style::from_egui(ui.style());
                                style.tab_bar.fill_tab_bar = true;
                                self.style = Some(style.clone());
                                style
                            }
                        };

                        // SAFETY: We're using interior mutability here. As long as show_inside doesn't access
                        // self.tree, this is a safe operation!
                        unsafe {
                            DockArea::new(&mut *self.tree.get()).style(style).show_inside(ui, self);
                        }
                    });
            }
        }
    }
}
