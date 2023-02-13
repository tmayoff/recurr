use std::collections::VecDeque;

use web_sys::{HtmlElement, MouseEvent};
use yew::{function_component, html, Callback, Html, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub current_page: i64,
    pub total_pages: i64,
    pub next_page: Option<Callback<()>>,
    pub prev_page: Option<Callback<()>>,
    pub goto_page: Option<Callback<i64>>,
}

#[function_component(Paginate)]
pub fn paginate(props: &Props) -> Html {
    let pages = paginate_pages(props.current_page, props.total_pages);
    let current_page = props.current_page;

    let next_page = {
        let cb = props.next_page.clone();
        match cb {
            Some(cb) => Callback::from(move |_| cb.emit(())),
            None => Callback::default(),
        }
    };

    let prev_page = {
        let cb = props.prev_page.clone();
        match cb {
            Some(cb) => Callback::from(move |_| cb.emit(())),
            None => Callback::default(),
        }
    };

    let goto_page = {
        let cb = props.goto_page.clone();
        match cb {
            Some(cb) => Callback::from(move |e: MouseEvent| {
                let target = e.target_dyn_into::<HtmlElement>().unwrap();
                let page = target
                    .get_attribute("data-page")
                    .unwrap_or("1".to_string())
                    .parse()
                    .unwrap();

                cb.emit(page)
            }),
            None => Callback::default(),
        }
    };

    html! {
        <nav class="pagination is-small is-centered" role="navigation" aria-label="pagination">
            if props.current_page == 1 {
                <a class="pagination-previous is-disabled">{"Prev"}</a>
            } else {
                <a class="pagination-previous" onclick={prev_page}>{"Prev"}</a>
            }
            if props.current_page == props.total_pages {
                <a class="pagination-next is-disabled">{"Next"}</a>
            } else {
                <a class="pagination-next" onclick={next_page}>{"Next"}</a>
            }
            <ul class="pagination-list">
            {
                pages.into_iter().map(|p| {
                    html!{
                        <li>
                            if current_page.to_string() == p {
                                <a class="pagination-link is-current" aria-label={format!("Goto page {p}")}>{p}</a>
                            } else {
                                <a onclick={goto_page.clone()} data-page={p.clone()} class="pagination-link" aria-label={format!("Goto page {p}")}>{p}</a>
                            }
                        </li>
                    }
                }).collect::<Html>()
            }
            </ul>
        </nav>
    }
}

fn paginate_pages(current_page: i64, page_count: i64) -> VecDeque<String> {
    const GAP: &str = "...";
    let center = vec![
        current_page - 2,
        current_page - 1,
        current_page,
        current_page + 1,
        current_page + 2,
    ];
    let mut center_deque: VecDeque<String> = center
        .iter()
        .filter(|&p| *p > 1i64 && *p < page_count)
        .map(i64::to_string)
        .collect();
    let include_three_left = current_page == 5;
    let include_three_right = current_page == page_count - 4;
    let include_left_dots = current_page > 5;
    let include_right_dots = current_page < page_count - 4;

    if include_three_left {
        center_deque.push_front("2".into());
    }
    if include_three_right {
        center_deque.push_back((page_count - 1i64).to_string());
    }
    if include_left_dots {
        center_deque.push_front(GAP.into());
    }
    if include_right_dots {
        center_deque.push_back(GAP.into());
    }
    center_deque.push_front("1".into());
    if page_count > 1i64 {
        center_deque.push_back(page_count.to_string());
    }
    center_deque
}
