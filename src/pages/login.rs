use yew::prelude::*;

use crate::components::input::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <div class="container">
            <div class="row min-vh-100 justify-content-center align-items-center">
                <div class="col-md-4">
                    <form>
                        <div class="mb-3">
                            <Input input_type="text" name="username" label="Username" />
                        </div>
                        <div class="mb-3">
                            <Input input_type="password" name="password" label="Password" />
                        </div>
                        <button type="submit">{"Login"}</button>
                    </form>
                </div>
            </div>
        </div>
    }
}
