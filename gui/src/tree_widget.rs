use iced::widget::{column, container, horizontal_space, row, text, Column, Space};
use iced::{border, Theme};
use iced::{Border, Color, Element};
use rsb_lib::VarNode;

use crate::message::Message;

pub fn adapter(node: VarNode) -> Vec<Vec<(char, NodeKind)>>{
    let depth = node.depth();
    println!("Depth: {}", depth);
    let mut grid: Vec<Vec<(char, NodeKind)>> = Vec::new();

    let mut queue: Vec<(VarNode, NodeKind)> = vec![(node, NodeKind::Root)];

    while !queue.is_empty() {
        let mut new_queue = Vec::new();
        let mut row = Vec::new();
        for (current_node, kind) in queue.into_iter() {
            row.push((current_node.char_repr(), kind));
            if let Some(left) = current_node.get_left() {
                new_queue.push((left, NodeKind::Left))
            };
            if let Some(right) = current_node.get_right() {
                new_queue.push((right, NodeKind::Right))
            };
        }
        queue = new_queue;
        grid.push(row);
    }
    return grid
}

pub struct VizTree {
    scale: f32,
}

impl VizTree {
    const NODE_SIZE: f32 = 100.0;
    const NODE_RADIUS: f32 = 20.0;
    const NODE_MARGIN: f32 = 10.0;
    const TEXT_SIZE: f32 = 60.0;

    pub fn new(scale: f32) -> Self {
        Self { scale }
    }

    fn node_style(&self, kind: NodeKind) -> container::Style {
        container::Style {
            background: Some(Color::BLACK.into()),
            border: Border {
                radius: kind.get_border(self.scaled_radius()),
                ..Border::default()
            },
            ..container::Style::default()
        }
    }

    fn scaled_radius(&self) -> f32 {
        self.scale * Self::NODE_RADIUS
    }

    fn scaled_size(&self) -> f32 {
        self.scale * Self::NODE_SIZE
    }

    fn scaled_margin(&self) -> f32 {
        self.scale * Self::NODE_MARGIN
    }

    fn scaled_text_size(&self) -> f32 {
        self.scale * Self::TEXT_SIZE
    }

    fn node(&self, kind: NodeKind, symbol: char) -> Element<Message> {
        container(text(symbol).size(self.scaled_text_size()))
            .center(self.scaled_size())
            .style(match kind {
                NodeKind::Left => NodeKind::left_node,
                NodeKind::Right => NodeKind::right_node,
                NodeKind::Root => NodeKind::root_node,
            })
            .into()
    }

    fn padder(&self, number: f32) -> Space {
        Space::with_width(self.scaled_size() * number)
    }

    pub fn test(&self) -> Column<'_, Message> {
        column![
            row![self.padder(0.5), self.node(NodeKind::Root, '&')].spacing(self.scaled_margin()),
            row![
                self.node(NodeKind::Left, '0'),
                self.node(NodeKind::Right, '1')
            ]
            .spacing(self.scaled_margin()),
        ]
        .spacing(self.scaled_margin())
        .padding(20)
    }
}

impl Default for VizTree {
    fn default() -> Self {
        Self { scale: 1.0 }
    }
}

enum NodeKind {
    Left,
    Right,
    Root,
}

impl NodeKind {
    fn get_border(&self, rad: f32) -> border::Radius {
        match self {
            Self::Left => border::radius(rad).top_right(0.),
            Self::Right => border::radius(rad).top_left(0.),
            Self::Root => border::radius(rad),
        }
    }

    pub fn left_node(theme: &Theme) -> container::Style {
        let color = theme.extended_palette().background.weak.color;

        container::Style {
            background: Some(color.into()),
            border: Border {
                radius: border::radius(20).top_right(0),
                ..Border::default()
            },
            ..container::Style::default()
        }
    }

    pub fn right_node(theme: &Theme) -> container::Style {
        let color = theme.extended_palette().background.weak.color;

        container::Style {
            background: Some(color.into()),
            border: Border {
                radius: border::radius(20).top_left(0),
                ..Border::default()
            },
            ..container::Style::default()
        }
    }

    pub fn root_node(theme: &Theme) -> container::Style {
        let color = theme.extended_palette().background.weak.color;

        container::Style {
            background: Some(color.into()),
            border: Border {
                radius: border::radius(20),
                ..Border::default()
            },
            ..container::Style::default()
        }
    }
}
