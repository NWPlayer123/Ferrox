use egui_extras::{Column, TableBuilder};

pub struct AssemblyTab;

impl AssemblyTab {
    pub fn update(&mut self, ui: &mut egui::Ui) {
        // This is currently placeholder data. TODO: add actual state? First let's see how performant it is
        // without caching gui elements
        let lookup = [
            (
                ".text:80005C68",
                "# int __fastcall main(int argc, const char **argv, const char **envp)",
            ),
            (".text:80005C68", "                 .globl main"),
            (
                ".text:80005C68",
                " main:                                   # CODE XREF: __start+154↑p",
            ),
            (".text:80005C68", "                 stwu      r1, -0x10(r1)"),
            (".text:80005C6C", "                 mflr      r0"),
            (".text:80005C70", "                 stw       r0, 0x14(r1)"),
            (".text:80005C74", "                 stw       r31, 0xC(r1)"),
            (".text:80005C78", "                 bl        marioStInit"),
            (".text:80005C7C", "                 bl        marioStMain"),
            (".text:80005C80", ""),
            (
                ".text:80005C80",
                " loc_80005C80:                           # CODE XREF: main+50↓j",
            ),
            (".text:80005C80", "                 bl        OSGetTick"),
            (".text:80005C84", "                 mr        r31, r3"),
            (".text:80005C88", "                 bl        DEMOBeforeRender"),
            (".text:80005C8C", "                 bl        marioStDisp"),
            (".text:80005C90", "                 bl        marioStMain"),
            (".text:80005C94", "                 bl        OSGetTick"),
            (
                ".text:80005C98",
                "                 lwz       r4, (gp - 0x800D6CA0)(r13) # marioSt",
            ),
            (".text:80005C9C", "                 subf      r0, r31, r3"),
            (
                ".text:80005CA0",
                "                 stw       r0, (dword_8009C744 - 0x8009B430)(r4)",
            ),
            (".text:80005CA4", "                 bl        DEMODoneRender"),
            (".text:80005CA8", "                 bl        OSGetTick"),
            (
                ".text:80005CAC",
                "                 lwz       r4, (gp - 0x800D6CA0)(r13) # marioSt",
            ),
            (".text:80005CB0", "                 subf      r0, r31, r3"),
            (
                ".text:80005CB4",
                "                 stw       r0, (dword_8009C748 - 0x8009B430)(r4)",
            ),
            (".text:80005CB8", "                 b         loc_80005C80"),
            (".text:80005CB8", "# End of function main"),
        ];

        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

        egui::ScrollArea::both().show(ui, |ui| {
            TableBuilder::new(ui).column(Column::auto()).column(Column::remainder().clip(true)).body(
                |body| {
                    body.rows(20.0, lookup.len(), |mut row| {
                        let index = row.index();
                        row.col(|ui| {
                            ui.label(egui::RichText::new(lookup[index].0).size(14.0));
                        });
                        row.col(|ui| {
                            ui.label(egui::RichText::new(lookup[index].1).size(14.0));
                        });
                    });
                },
            );
        });
    }
}
