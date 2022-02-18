use kayak_core::styles::{PositionType, Style, StyleProp, Units};
use kayak_core::{Bound, Color, EventType, OnEvent, VecTracker};
use kayak_render_macros::{constructor, use_state};

use crate::core::{rsx, widget, MutableBound, WidgetProps};

use crate::widgets::{Background, Button, Text};

// TODO: Remove if unneeded
#[derive(Clone, PartialEq)]
pub enum InspectData {
    None,
    Data(Vec<String>),
}

/// Props used by the [`Inspector`] widget
#[derive(WidgetProps, Default, Debug, PartialEq, Clone)]
pub struct InspectorProps {
    #[prop_field(Styles)]
    pub styles: Option<Style>,
}

#[widget]
/// A widget that displays debug data for inspected widgets
///
/// "Inspected widgets" refers to the last clicked widget.
///
/// # Props
///
/// __Type:__ [`InspectorProps`]
///
/// | Common Prop | Accepted |
/// | :---------: | :------: |
/// | `children`  | ❌        |
/// | `styles`    | ✅        |
/// | `on_event`  | ❌        |
/// | `focusable` | ❌        |
///
pub fn Inspector(props: InspectorProps) {
    let (inspect_data, set_inspect_data, _) = use_state!(Vec::<String>::new());

    let background_styles = Some(Style {
        background_color: StyleProp::Value(Color::new(0.125, 0.125, 0.125, 1.0)),
        border_radius: StyleProp::Value((0.0, 0.0, 0.0, 0.0)),
        position_type: StyleProp::Value(PositionType::SelfDirected),
        left: StyleProp::Value(Units::Stretch(1.0)),
        top: StyleProp::Value(Units::Stretch(0.0)),
        bottom: StyleProp::Value(Units::Stretch(0.0)),
        width: StyleProp::Value(Units::Pixels(200.0)),
        ..props.styles.clone().unwrap_or_default()
    });

    let last_clicked = context.get_last_clicked_widget();
    context.bind(&last_clicked);

    let last_clicked_value = last_clicked.get();
    let (id, _) = last_clicked_value.into_raw_parts();

    let mut parent_id_move = None;
    if let Some(layout) = context.get_layout(&last_clicked_value) {
        if let Some(node) = context.get_node(&last_clicked_value) {
            let mut data = Vec::new();

            if let Some(name) = context.get_name(&last_clicked_value) {
                data.push(format!("Name: {}", name));
            }

            data.push(format!("ID: {}", id));
            data.push(format!("X: {}", layout.posx));
            data.push(format!("Y: {}", layout.posy));
            data.push(format!("Width: {}", layout.width));
            data.push(format!("Height: {}", layout.height));
            data.push(format!(
                "RenderCommand: \n{:#?}",
                node.resolved_styles.render_command
            ));
            data.push(format!("Height: \n{:#?}", node.resolved_styles.height));

            if let Some(parent_id) = context.get_valid_parent(last_clicked_value) {
                parent_id_move = Some(parent_id);
                if let Some(layout) = context.get_layout(&parent_id) {
                    data.push(format!("_________Parent_________"));
                    if let Some(name) = context.get_name(&parent_id) {
                        data.push(format!("Name: {}", name));
                    }
                    data.push(format!("X: {}", layout.posx));
                    data.push(format!("Y: {}", layout.posy));
                    data.push(format!("Width: {}", layout.width));
                    data.push(format!("Height: {}", layout.height));
                }
            }
            set_inspect_data(data);
        }
    }

    let handle_button_events = Some(OnEvent::new(move |_, event| match event.event_type {
        EventType::Click(..) => last_clicked.set(parent_id_move.unwrap()),
        _ => {}
    }));

    rsx! {
        <Background styles={background_styles}>
            {VecTracker::from(inspect_data.iter().map(|data| {
                constructor! {
                    <Text content={data.clone().to_string()} size={12.0} />
                }
            }))}
            <Button>
                <Text content={"Go Up".into()} size={12.0} on_event={handle_button_events} />
            </Button>
        </Background>
    }
}
