use render::html;

fn main() {
    html! {
      <div>
        <h1>{"A list"}</h1>
        <hr />
        <ul>
          <li>{"1"}</li>
          <li>
        </ul>
      </div>
    };
}
