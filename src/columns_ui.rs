///Column UI refers to two columns where the user
/// can drag and drop labels between each column.
/// This is used for the items inventory, and for assigning activities.

pub struct ColumnUI<'a, T> {
    pub first: &'a mut Vec<T>,
    pub second: &'a mut Vec<T>,
    pub first_name: String,
    pub second_name: String,
}

pub trait ColumnItem {
    fn tooltip(&self) -> String;
    fn name(&self) -> &str;
}

//Determines which column the item is in i.e equipped or inventory
// and its position relative to other items in the list (top -> bottom)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ColumnLocation {
    first: bool,
    row: usize,
}

impl ColumnLocation {
    pub fn new(first: bool, row: usize) -> Self {
        Self { first, row }
    }

    pub fn first(&self) -> bool {
        self.first
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn set_row(&mut self, row: usize) {
        self.row = row;
    }
}

impl<'a, T: Clone + ColumnItem> ColumnUI<'a, T> {
    //Category is used to make unique ids.
    pub fn show(&mut self, uis: &mut [notan_egui::Ui]) {
        let frame = notan_egui::Frame::default().inner_margin(1.0);
        for i in 0..2 {
            let mut from_location: Option<ColumnLocation> = None;
            let mut to_location: Option<ColumnLocation> = None;

            let ui = &mut uis[i];
            let (is_first_column, current_list, heading) = if i == 0 {
                (true, self.first.clone(), self.first_name.as_str())
            } else {
                (false, self.second.clone(), self.second_name.as_str())
            };

            let (_, dropped_location) = ui.dnd_drop_zone::<ColumnLocation, ()>(frame, |ui| {
                ui.heading(heading);

                for (row, item) in current_list.iter().enumerate() {
                    let item_id = notan_egui::Id::new((is_first_column, row, item.name()));
                    let item_loc = ColumnLocation::new(is_first_column, row);

                    let tooltip = item.tooltip();

                    let response = ui
                        .dnd_drag_source(item_id, item_loc, |ui| ui.label(item.name()))
                        .response
                        .on_hover_text(tooltip);

                    if let (Some(pointer), Some(hovered_location)) = (
                        ui.input(|i| i.pointer.interact_pos()),
                        response.dnd_hover_payload::<ColumnLocation>(),
                    ) {
                        let rect = response.rect;
                        let stroke = notan_egui::Stroke::new(
                            2.0,
                            notan_egui::Color32::from_rgb(250, 250, 250),
                        );
                        let insert_row_index = if *hovered_location == item_loc {
                            row
                        } else if pointer.y < rect.center().y {
                            ui.painter().hline(rect.x_range(), rect.top(), stroke);
                            row
                        } else {
                            ui.painter().hline(rect.x_range(), rect.bottom(), stroke);
                            row + 1
                        };

                        if let Some(dragged_location) =
                            response.dnd_release_payload::<ColumnLocation>()
                        {
                            from_location = Some(*dragged_location);
                            to_location =
                                Some(ColumnLocation::new(is_first_column, insert_row_index));
                        }
                    }
                }
            });

            if let Some(dragged_location) = dropped_location {
                from_location = Some(*dragged_location);
                to_location = Some(ColumnLocation::new(is_first_column, usize::MAX));
            }

            if let (Some(from), Some(mut to)) = (from_location, to_location) {
                if from.first() == to.first() {
                    let offset = (from.row() < to.row()) as usize;
                    to.set_row(to.row() - offset);
                }

                let from_list = if from.first() {
                    &mut self.first
                } else {
                    &mut self.second
                };

                let item = from_list.remove(from.row());

                let to_list = if to.first() {
                    &mut self.first
                } else {
                    &mut self.second
                };

                to.set_row(to.row().min(to_list.len()));
                to_list.insert(to.row(), item);
            }
        }
    }
}
