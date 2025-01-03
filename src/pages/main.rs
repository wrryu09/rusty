use yew::prelude::*;
use crate::{api::get_items, types::{ObjectList, TestObject}};
use anyhow:: Error;
use wasm_bindgen_futures::spawn_local;

struct State{
    items: Vec<TestObject>,
    item_list: Vec<ObjectList>,
    get_items_error: Option<Error>,
    get_items_loaded: bool,
}

pub struct MainPage{
    state: State,
}

pub enum Msg{
    AddToList(i32),
    GetItems,
    GetItemsSuccess(Vec<TestObject>),
    GetItemsError(Error)
}

impl Component for MainPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let items: Vec<TestObject> = vec![];
        let item_list: Vec<ObjectList> = vec![];

        ctx.link().send_message(Msg::GetItems);

        Self{
            state: State { 
                items,
                item_list, 
                get_items_error:None,
                get_items_loaded: false
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Msg::GetItems=>{
                self.state.get_items_loaded = false;
                let callback = ctx.link().callback(|result: Result<Vec<TestObject>, gloo_net::Error>| {
                    match result {
                        Ok(items) => Msg::GetItemsSuccess(items),
                        Err(err) => Msg::GetItemsError(err.into()),
                    }
                });

                spawn_local(async move {
                    let res = get_items().await;
                    callback.emit(res);
                });

                true
            }

            Msg::GetItemsSuccess(items)=>{
                self.state.items = items;
                self.state.get_items_loaded = true;
                true
            }

            Msg::GetItemsError(err)=>{
                self.state.get_items_error = Some(err);
                self.state.get_items_loaded = true;
                true
            }

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
        
        if !self.state.get_items_loaded{
            html! {
                <div>{"Loading ..."}</div>
            }
        }else if let Some(_) = self.state.get_items_error{
            html! {
                <div>{"Err"}</div>
            }
        }else{
            html! { 
                <div>
                    <span>{format!("item amount: {:.2}", list_quantity)}</span>
                    <span>{items}</span>
                </div>
                }
        }
    }
}
