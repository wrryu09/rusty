use yew::prelude::*;

struct TestObject{
    id: i32,
    name: String,
    desc: String
}

struct State{
    items: Vec<TestObject>
}

pub struct MainPage{
    state: State,
}

impl Component for MainPage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let items: Vec<TestObject> = vec![
            TestObject{
                id: 0,
                name: "firstObj".to_string(),
                desc: "first obj in mainpage".to_string(),
            },
            TestObject{
                id: 1,
                name: "scndObj".to_string(),
                desc: "second".to_string(),
            },
        ];

        Self{
            state: State { items, }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let items: Vec<Html> = self.state.items.iter().map(|obj:&TestObject| {
            html!{
                <div>
                    <p>{&obj.id}</p>
                    <p>{&obj.name}</p>
                    <p>{&obj.desc}</p>
                </div>
            }
        }).collect();
        html! { <span>{items}</span> }
    }
}
