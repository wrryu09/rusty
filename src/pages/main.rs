use yew::prelude::*;

#[derive(Clone)]

struct TestObject{
    id: i32,
    name: String,
    desc: String
}

struct ObjectList{
    object: TestObject,
    quantity: i32
}

struct State{
    items: Vec<TestObject>,
    item_list: Vec<ObjectList>
}

pub struct MainPage{
    state: State,
}

pub enum Msg{
    AddToList(i32)
}

impl Component for MainPage {
    type Message = Msg;
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
        let item_list = vec![];

        Self{
            state: State { 
                items,
                item_list, 
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Msg::AddToList(obj_id)=>{
                let obj = self.state.items.iter().find(|ob: &&TestObject| ob.id == obj_id).unwrap();

                let obj_list = self.state.item_list.iter_mut().find(|li: &&mut ObjectList| li.object.id == obj_id);

                if let Some(obj) = obj_list{
                    obj.quantity+=1;
                }else{
                    self.state.item_list.push(
                        ObjectList{
                            object: obj.clone(),
                            quantity:1,
                        }
                    );
                }
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let items: Vec<Html> = self.state.items.iter().map(|obj:&TestObject| {
            let object_id = obj.id;
            html!{
                <div>
                    <p>{&obj.id}</p>
                    <p>{&obj.name}</p>
                    <p>{&obj.desc}</p>
                    <button 
                    onclick={ctx.link().callback(move |_| Msg::AddToList(object_id.clone()))}>
                    { "Add To List" }
                    </button>
                </div>
            }
        }).collect();

        let list_quantity = self.state.item_list.iter().fold(0, |acc, objli: &ObjectList| acc+objli.quantity);
        html! { 
        <div>
            <span>{format!("item amount: {:.2}", list_quantity)}</span>
            <span>{items}</span>
        </div>
        }
    }
}
