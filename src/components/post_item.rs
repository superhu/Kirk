use crate::common::types;
use log::*;
use pulldown_cmark::{html, Options, Parser};
use stdweb::unstable::TryFrom;
use stdweb::web::Node;
use yew::prelude::*;
use yew::virtual_dom::VNode;

pub struct PostItem {
  post: types::Post,
  style: String,
  class: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
  #[props(required)]
  pub post: types::Post,
  pub class: String,
  pub style: String,
}

impl Component for PostItem {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {
      post: props.post,
      class: props.class,
      style: props.style,
    }
  }

  // fn mounted() -> ShouldRender {
  // }
  fn update(&mut self, _: Self::Message) -> ShouldRender {
    false
  }

  // fn change(&mut self, props: Self::Properties) -> ShouldRender {
  //   self.post = props.post;
  //   self.class = props.class;
  //   self.style = props.style;
  //   true
  // }
}

impl PostItem {
  fn parse_markdown(&self) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(self.post.body.as_str(), options);

    let mut html_str = String::new();
    html::push_html(&mut html_str, parser);
    trace!("{}", self.post.body);
    trace!("{}", html_str);
    html_str
    // markdown_to_html(&self.post.body, &ComrakOptions::default())
  }
  fn markdown_view(&self) -> Html<Self> {
    let render = js! {
      var div = document.createElement("div");
      div.innerHTML = @{self.parse_markdown()};
      return div;
    };
    if let Ok(node) = Node::try_from(render) {
      let vnode = VNode::VRef(node);
      vnode
    } else {
      html! {
        <div class="error">{"error"}</div>
      }
    }
  }
  fn header_view(&self) -> Html<Self> {
    html! {
      <div class="post-item__header">
      <div class="post-item__header__title">{&self.post.title}</div>
      <ul class="post-item__header__meta">
        <li class="post-item__header__author">
          <span style={format!("background-image: url({});", self.post.user.avatar_url)} class="post-item__header__avator"  />
          <span class="post-item__header__name"><a target="_blank" href={format!("{}",self.post.user.url)}>{&self.post.user.login}</a></span>
        </li>
        // <li class="post-item__header__created-at"><label>{"Created At: "}</label>{&self.post.created_at}</li>
        <li class="post-item__header__updated-at">
              <label><i class="iconfont icon-pencil" />{"Updated At: "}</label>
              <span>{&self.post.updated_at}</span>
         </li>
      </ul>
      {self.tags_view()}
      </div>
    }
  }
  fn tags_view(&self) -> Html<Self> {
    if self.post.labels.len() == 0 {
      html! {}
    } else {
      html! {
        <ul class="post-item__header__tags">
          {for self.post.labels.iter().map(|label| html! {
            <li key={&label.id}>
              <span class="tag" style=format!("background-color: #{}", label.color)>{&label.name}</span>
            </li>
          })}
        </ul>
      }
    }
  }
  fn body_view(&self) -> Html<Self> {
    html! {
      <section class=format!("post-item__body post-item__body--{}", self.post.id)>
        {self.markdown_view()}
      </section>
    }
  }
}
impl Renderable<PostItem> for PostItem {
  fn view(&self) -> Html<Self> {
    html! {
      <div
         class=format!("post-item {}", self.class)
         style=self.style
      >
        {self.header_view()}
        {self.body_view()}
      </div>
    }
  }
}