use yew::prelude::*;

const SPRITE_HEIGHT: u8 = 16u8;

#[derive(PartialEq)]
pub enum TableValue {
    None,
    String(AttrValue),
    Sprite(AttrValue, u16, u16, u16, u16, u16, u16),
}

impl ToHtml for TableValue {
    fn to_html(&self) -> Html {
        match self {
            TableValue::None => html!(),
            TableValue::String(string) => html!({ string }),
            TableValue::Sprite(image, x, y, width, height, sheet_width, sheet_height) => html!(
                <figure class={ format!("image") } style={ format!("background: url(/assets/{image}) calc(-{x}px * ({SPRITE_HEIGHT} / {height})) calc(-{y}px * ({SPRITE_HEIGHT} / {height})) / calc({sheet_width}px * ({SPRITE_HEIGHT} / {height})) calc({sheet_height}px * ({SPRITE_HEIGHT} / {height})); width: calc({width}px * ({SPRITE_HEIGHT} / {height})); height: calc({height}px * ({SPRITE_HEIGHT} / {height})); image-rendering: pixelated; display: inline-block;") } />
            ),
        }
    }
}

#[derive(PartialEq)]
pub enum TableAlign {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl ToString for TableAlign {
    fn to_string(&self) -> String {
        match self {
            TableAlign::TopLeft => "vertical-align: top; text-align: left;",
            TableAlign::TopCenter => "vertical-align: top; text-align: center;",
            TableAlign::TopRight => "vertical-align: top; text-align: right;",
            TableAlign::MiddleLeft => "vertical-align: middle; text-align: left;",
            TableAlign::MiddleCenter => "vertical-align: middle; text-align: center;",
            TableAlign::MiddleRight => "vertical-align: middle; text-align: right;",
            TableAlign::BottomLeft => "vertical-align: bottom; text-align: left;",
            TableAlign::BottomCenter => "vertical-align: bottom; text-align: center;",
            TableAlign::BottomRight => "vertical-align: bottom; text-align: right;",
        }
        .to_string()
    }
}

#[derive(PartialEq)]
pub struct TableCell {
    pub value: TableValue,
    pub align: TableAlign,
    pub rows: u8,
    pub columns: u8,
}
#[derive(Properties, PartialEq)]
pub struct TableProperties {
    pub header: Vec<Vec<TableCell>>,
    pub body: Vec<Vec<TableCell>>,
}

#[function_component]
pub fn Table(properties: &TableProperties) -> Html {
    html!(
        <table class="table is-fullwidth">
            <thead>
                {
                    properties.header.iter().map(|row|{
                        html!(
                            <tr>
                                {
                                    row.iter().map(|cell|{
                                        html!(
                                            <th style={ cell.align.to_string() } rowspan={ cell.rows.to_string() } colspan={ cell.columns.to_string() }>{ &cell.value }</th>
                                        )
                                    }).collect::<Html>()
                                }
                            </tr>
                        )
                    }).collect::<Html>()
                }
            </thead>
            <tbody>
                {
                    properties.body.iter().map(|row|{
                        html!(
                            <tr>
                                {
                                    row.iter().map(|cell|{
                                        html!(
                                            <td style={ cell.align.to_string() } rowspan={ cell.rows.to_string() } colspan={ cell.columns.to_string() }>{ &cell.value }</td>
                                        )
                                    }).collect::<Html>()
                                }
                            </tr>
                        )
                    }).collect::<Html>()
                }
            </tbody>
        </table>
    )
}
