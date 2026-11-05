use super::tabs::{open_on_double_click, paint_dots};
use super::{PhotocraftApp, Rect, Sense};

pub(super) fn start_screen(app: &mut PhotocraftApp, ui: &mut egui::Ui) {
    let area = ui.available_rect_before_wrap();
    paint_dots(ui, area, app.session.prefs().interface.show_canvas_dots);
    open_on_double_click(app, ui, area);

    // File › Open Recent, newest first (on the web there are no paths to reopen).
    let show_recent = app.session.prefs().interface.show_home_recent_files;
    let recent: Vec<String> =
        if cfg!(target_arch = "wasm32") || !show_recent { Vec::new() } else { app.ui.recent_files.iter().take(HOME_RECENT).cloned().collect() };

    let is_narrow = area.width() < 640.0;
    let pad_x = if is_narrow { 16.0 } else { 36.0 };
    let pad_y = if is_narrow { 24.0 } else { 54.0 };
    let rail_w = 170.0;

    if is_narrow {
        let card_w = (area.width() - 32.0).clamp(0.0, 520.0);
        let card = Rect::from_center_size(area.center(), egui::vec2(card_w, (area.height() - 40.0).max(0.0)));
        ui.scope_builder(egui::UiBuilder::new().max_rect(card), |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                render_start_actions_row(app, ui, card.width());
                ui.add_space(16.0);
                render_start_content(app, ui, &recent, card.width());
            });
        });
    } else {
        let rail_rect = Rect::from_min_size(egui::pos2(area.left() + pad_x, area.top() + pad_y), egui::vec2(rail_w, area.height() - pad_y * 2.0));
        let content_x = rail_rect.right() + 36.0;
        let content_rect = Rect::from_min_max(egui::pos2(content_x, area.top() + pad_y), egui::pos2(area.right() - pad_x, area.bottom() - 16.0));

        ui.scope_builder(egui::UiBuilder::new().max_rect(rail_rect), |ui| {
            render_start_rail(app, ui, rail_rect.width());
        });

        ui.scope_builder(egui::UiBuilder::new().max_rect(content_rect), |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                render_start_content(app, ui, &recent, content_rect.width());
            });
        });
    }
}

fn open_new_document(app: &mut PhotocraftApp) {
    let fields = app.new_document_fields();
    app.ui.open_dialog(crate::state::DialogKind::NewDocument, fields);
}

fn render_start_actions_row(app: &mut PhotocraftApp, ui: &mut egui::Ui, avail_w: f32) {
    let btn_w = ((avail_w - 8.0) / 2.0).clamp(0.0, 180.0);

    ui.horizontal(|ui| {
        if crate::widgets::primary_button(ui, "New file", btn_w).clicked() {
            open_new_document(app);
        }
        if crate::widgets::secondary_button(ui, "Open…", btn_w).clicked() {
            let _ = app.open_dialog_file();
        }
    });
}

fn render_start_rail(app: &mut PhotocraftApp, ui: &mut egui::Ui, rail_w: f32) {
    let t = crate::theme::Tokens::get(ui.ctx());
    let button_w = rail_w.min(150.0);

    ui.add_space(8.0);
    if crate::widgets::primary_button(ui, "New file", button_w).clicked() {
        open_new_document(app);
    }
    ui.add_space(8.0);
    if crate::widgets::secondary_button(ui, "Open…", button_w).clicked() {
        let _ = app.open_dialog_file();
    }

    ui.add_space(16.0);
    let sep_w = 150.0f32.min(rail_w);
    let (sep_rect, _) = ui.allocate_exact_size(egui::vec2(sep_w, 1.0), Sense::hover());
    ui.painter().line_segment([sep_rect.min, egui::pos2(sep_rect.max.x, sep_rect.min.y)], egui::Stroke::new(1.0, t.separator));
    ui.add_space(14.0);

    // Home hides the tab strip and options bar, so this entry is the way back to open documents.
    let (home_rect, home_resp) = ui.allocate_exact_size(egui::vec2(rail_w, 32.0), Sense::click());
    home_resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, tl!("Home")));
    if !app.session.documents().is_empty() && home_resp.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
        app.ui.chrome.home = None;
    }
    let active_bar = Rect::from_min_size(egui::pos2(home_rect.left(), home_rect.center().y - 7.0), egui::vec2(1.5, 14.0));
    ui.painter().rect_filled(active_bar, 1.0, t.accent);
    let house_box = Rect::from_center_size(egui::pos2(home_rect.left() + 20.0, home_rect.center().y), egui::vec2(16.0, 16.0));
    crate::icons::paint(ui, house_box, "house", 16.0, t.text);
    ui.painter().text(egui::pos2(home_rect.left() + 34.0, home_rect.center().y), egui::Align2::LEFT_CENTER, tl!("Home"), crate::theme::semibold(13.5), t.text);

    ui.add_space(4.0);
    let (learn_rect, learn_resp) = ui.allocate_exact_size(egui::vec2(rail_w, 32.0), Sense::click());
    let learn_resp = learn_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
    learn_resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, tl!("Learn")));
    if learn_resp.hovered() {
        ui.painter().rect_filled(learn_rect, 4.0, t.hover);
    }
    let learn_color = if learn_resp.hovered() { t.text } else { t.text_dim };
    let sparkle_box = Rect::from_center_size(egui::pos2(learn_rect.left() + 20.0, learn_rect.center().y), egui::vec2(16.0, 16.0));
    crate::icons::paint(ui, sparkle_box, "sparkles", 16.0, learn_color);
    ui.painter().text(
        egui::pos2(learn_rect.left() + 34.0, learn_rect.center().y),
        egui::Align2::LEFT_CENTER,
        tl!("Learn"),
        crate::theme::semibold(13.5),
        learn_color,
    );
    if learn_resp.clicked() {
        crate::links::open(app, ui.ctx(), crate::links::TUTORIALS);
    }

    // External community links are separated from the actions used to start editing.
    let bottom_h = 100.0;
    let spacer = (ui.available_height() - bottom_h - 20.0).max(24.0);
    ui.add_space(spacer);
    let (b_sep_rect, _) = ui.allocate_exact_size(egui::vec2(sep_w, 1.0), Sense::hover());
    ui.painter().line_segment([b_sep_rect.min, egui::pos2(b_sep_rect.max.x, b_sep_rect.min.y)], egui::Stroke::new(1.0, t.separator));
    ui.add_space(8.0);

    render_rail_link(app, ui, rail_w, tl!("Website"), "link", crate::links::APP_PAGE);
    render_rail_link(app, ui, rail_w, tl!("Discord"), "message-square", crate::links::DISCORD);
    render_rail_link(app, ui, rail_w, tl!("GitHub"), "link-2", crate::links::GITHUB);
}

fn render_rail_link(app: &mut PhotocraftApp, ui: &mut egui::Ui, rail_w: f32, label: &str, icon_name: &str, url: &str) {
    let t = crate::theme::Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(rail_w, 28.0), Sense::click());
    let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand).on_hover_text(url);
    if resp.hovered() {
        ui.painter().rect_filled(rect, 4.0, t.hover);
    }
    let color = if resp.hovered() { t.text } else { t.text_dim };
    let icon_box = Rect::from_center_size(egui::pos2(rect.left() + 20.0, rect.center().y), egui::vec2(14.0, 14.0));
    crate::icons::paint(ui, icon_box, icon_name, 14.0, color);
    ui.painter().text(egui::pos2(rect.left() + 34.0, rect.center().y), egui::Align2::LEFT_CENTER, label, crate::theme::medium(12.0), color);
    if resp.clicked() {
        crate::links::open(app, ui.ctx(), url);
    }
}

fn render_start_content(app: &mut PhotocraftApp, ui: &mut egui::Ui, recent: &[String], content_w: f32) {
    let t = crate::theme::Tokens::get(ui.ctx());
    ui.add_space(8.0);

    ui.label(egui::RichText::new(tl!("PhotoCraft")).font(crate::theme::semibold(20.0)).color(t.text));
    ui.add_space(18.0);

    let (sep_r, _) = ui.allocate_exact_size(egui::vec2(content_w, 1.0), Sense::hover());
    ui.painter().line_segment([sep_r.min, egui::pos2(sep_r.max.x, sep_r.min.y)], egui::Stroke::new(1.0, t.separator));
    ui.add_space(24.0);

    if recent.is_empty() {
        render_first_run_content(app, ui, content_w);
    } else {
        crate::home_recent_ui::show_home_recent(app, ui, recent, content_w);
    }

    ui.add_space(24.0);
}

fn render_first_run_content(app: &mut PhotocraftApp, ui: &mut egui::Ui, content_w: f32) {
    let t = crate::theme::Tokens::get(ui.ctx());

    // Make the next step explicit instead of relying on the entire drop area being clickable.
    let content_width = content_w.max(0.0);
    let drop_w = 540.0_f32.min((content_width - 24.0).max(0.0));
    let drop_h = 200.0;
    let avail_h = ui.available_height();
    ui.add_space(((avail_h - drop_h) / 2.0 - 70.0).max(24.0));

    let pad_x = ((content_width - drop_w) / 2.0).max(0.0);
    ui.horizontal(|ui| {
        ui.add_space(pad_x);
        let (drop_r, drop_resp) = ui.allocate_exact_size(egui::vec2(drop_w, drop_h), Sense::click());
        let drop_resp = drop_resp.on_hover_cursor(egui::CursorIcon::PointingHand);
        let hovered = drop_resp.hovered();

        ui.painter().rect_filled(drop_r, 8.0, if hovered { t.hover } else { t.card });
        ui.painter().rect_stroke(
            drop_r,
            8.0,
            egui::Stroke::new(1.0, if hovered { t.accent } else { t.card_border }),
            egui::StrokeKind::Inside,
        );

        let icon_box = Rect::from_center_size(egui::pos2(drop_r.center().x, drop_r.top() + 42.0), egui::vec2(40.0, 40.0));
        crate::icons::paint(ui, icon_box, "image", 34.0, if hovered { t.text } else { t.text_dim });
        ui.painter().text(
            egui::pos2(drop_r.center().x, drop_r.top() + 82.0),
            egui::Align2::CENTER_CENTER,
            tl!("No recent files"),
            crate::theme::semibold(15.0),
            t.text,
        );

        // Show the primary action by name; dropping a file remains an optional shortcut.
        let button_width = drop_r.width().min(180.0);
        let button_rect = Rect::from_center_size(egui::pos2(drop_r.center().x, drop_r.top() + 126.0), egui::vec2(button_width, 30.0));
        let button_resp = ui.interact(button_rect, drop_resp.id.with("open"), Sense::click());
        button_resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, tl!("Open…")));
        let button_fill = if button_resp.is_pointer_button_down_on() {
            t.accent.gamma_multiply(0.82)
        } else if button_resp.hovered() {
            t.accent.gamma_multiply(0.9)
        } else {
            t.accent
        };
        ui.painter().rect_filled(button_rect, 6.0, button_fill);
        ui.painter().text(button_rect.center(), egui::Align2::CENTER_CENTER, tl!("Open…"), crate::theme::semibold(13.0), t.primary_text);

        let drop_msg = start_screen_drop_hint(app);
        let job = egui::text::LayoutJob::simple(drop_msg.to_string(), egui::FontId::proportional(11.5), t.text_dim, (drop_r.width() - 24.0).max(0.0));
        let galley = ui.painter().layout_job(job);
        ui.painter().galley(egui::pos2(drop_r.center().x - galley.size().x / 2.0, drop_r.top() + 151.0), galley, t.text_dim);

        if button_resp.clicked() || drop_resp.clicked() {
            let _ = app.open_dialog_file();
        }
    });

    ui.add_space(14.0);
    ui.horizontal(|ui| {
        ui.add_space(pad_x);
        if crate::widgets::primary_button(ui, "New file", 150.0).clicked() {
            open_new_document(app);
        }
        ui.add_space(8.0);
        if crate::widgets::secondary_button(ui, "Open…", 150.0).clicked() {
            let _ = app.open_dialog_file();
        }
    });
}

/// Wayland has no native file drops (winit 0.30, #386), so the hint offers File › Open and paste.
pub(crate) fn start_screen_drop_hint(app: &PhotocraftApp) -> std::borrow::Cow<'static, str> {
    if app.services.is_wayland {
        crate::i18n::fmt(tl!("Use File › Open, or paste a copied image with {paste}."), &[("paste", &crate::notices::paste_hint(app))]).into()
    } else {
        tl!("Drop an image or PSD anywhere to open it.").into()
    }
}

/// Recent files listed on the Home screen.
const HOME_RECENT: usize = 11;
