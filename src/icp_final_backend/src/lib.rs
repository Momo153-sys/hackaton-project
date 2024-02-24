
use candid::types::number::Nat;
use std::cell::RefCell;
use std::borrow::Borrow;

use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext, TransformFunc,
};
use serde_json::from_str;
use ic_cdk_macros::{query,update};
use candid::{CandidType,Deserialize};

thread_local! {
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0_u32));
}

mod check{
    pub static mut CONDITION:usize =1;
    pub static mut TASKS: Vec<String> = vec![];
}

#[derive(CandidType,Debug,Default)]
struct Nft{
    owner:String,
    id:u64,
}
impl Nft{
    fn new(owner:String,id:u64) -> Self{
        let new_owner = Nft{
            owner,
            id,
        };
        new_owner
    }
    fn tranfer(&mut self,to:String) -> String{
        let mut result = String::new();
        unsafe{
            if check::CONDITION==0{
                result ="Weather conditions are not met so not transfer".to_string();
            }else{
                result = format!("Transfer from {} with ID {} to {}"
            ,self.owner,self.id,to);
            }
        }
        result
    }
}
#[derive(CandidType,Debug,Default)]
struct Task{
    tasks:Vec<String>,
}

impl Task{
    fn new()->Self{
        let new_task = Task{
            tasks:vec![
                "Clean the Bathroom".to_string(),
                "Mow the Land".to_string(),
                "Do the laundry".to_string(),
            ],
        };
        new_task
    }
    fn add_task(&mut self, description:String) -> Task{
        let mut new_vector = self.tasks.clone();
        new_vector.push(description);    
        
            let new_task =Task{
            tasks:new_vector,
        } ;
        new_task
}

    fn erase_task(&mut self,description:String) -> Task{
        let mut new_vector = self.tasks.clone();
        if let Some(index) = new_vector.iter().position(|x| x == &description) {
            new_vector.remove(index);
        } 
           
        
            let new_task =Task{
            tasks:new_vector,
        } ;
        new_task
    }
}




#[update]
async fn display_data(api_key:String) -> String {
    let api_endpoint = "http://api.openweathermap.org/data/2.5/weather?q=";
   let city = "Istanbul";
   let country_code = "TR";
   
   let final_url =format!("{}{},{}&APPID={}",
   api_endpoint,city,country_code,api_key);
  
   let request_headers = vec![
    HttpHeader {
        name: "Host".to_string(),
        value: format!("api.openweathermap.org:443"),
    },

   ];

let request = CanisterHttpRequestArgument {
    url: final_url,
    method: HttpMethod::GET,
  body:None,
  max_response_bytes:None,
    transform: Some(TransformContext {
        // The "method" parameter needs to the same name as the function name of your transform function
        function: TransformFunc(candid::Func {
            principal: ic_cdk::api::id(),
            method: "transform".to_string(),
        }),
        // The "TransformContext" function does need a context parameter, it can be empty
        context: vec![],
    }),
    headers: request_headers,
};
let cycles = 1_603_115_200;
match http_request(request, cycles).await {
    Ok((response,)) => {
       let body_string= String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.");
       let json_value:serde_json::Value =
    from_str(&body_string).expect("Error in turning into Json Value");
    unsafe{
        if json_value["main"]["temp"] == 285{
            check::CONDITION=0;
        }
    }
    body_string
    }
    Err((r, m)) => {
        unsafe{
            check::CONDITION=0;
        }
        let message =
            format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

        //Return the error as a string and end the method
        message
    }
}
}


#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
      
    ];

    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
    };

    if res.status == 200u64 {
        res.body = raw.response.body;
    } else {
        ic_cdk::api::print(format!("Received an error from coinbase: err = {:?}", raw));
    }
 res 
}


#[update]
fn nft_transfer(receiver:String) -> String{
let mut new_owner = Nft::new("Mohamed".to_string(),2307);
let mut answer = String::new();
unsafe{
    if check::CONDITION==0{
        answer = "Weather Data connection is not working or Weather Data 
        condition is not met( Istanbul temperature should be 
            above 280 K )".to_string();
    }
else{
    answer = new_owner.tranfer(receiver);
} 
answer

}}




#[update]
fn initialize_list() -> String{
    let new_list = Task::new();
    unsafe{
        check::TASKS = new_list.tasks.clone();
    }
    let mut result =String::new();
    for i in &new_list.tasks{
        result.push_str(i);
        result += "\n";
    }
    result
}

#[update]
fn add_task(description:String) -> String{
    let mut result = String::new();
    let mut  list = vec![];
    unsafe{
        list = check::TASKS.clone();
    }
    if let Some(index) = list.iter().position(|x| x == &description) {
        result = "Task already There".to_string();
    } else {
        
        list.push(description);
        result = "Task added successfully".to_string();
    }
    unsafe{
        check::TASKS = list.clone();
    }
    result

}


#[update]
fn erase_task(description:String) -> String{
    let mut result = String::new();
    let mut  list = vec![];
    unsafe{
        list = check::TASKS.clone();
    }
    if let Some(index) = list.iter().position(|x| x == &description) {
        list.remove(index);
        result = "Task erased successfully".to_string();
    } else {
        result = "Task not found".to_string();
    }
    unsafe{
        check::TASKS = list.clone();
    }
    result
}


#[update]
fn display_list() ->String{
    let  result:Vec<String>;
    unsafe{
        result = check::TASKS.clone();
    }
    let mut output = "The Task list is: \n".to_string();
    for x in &result{
        output.push_str(x);
        output += "\n";
    }
    output

}


/// Get the value of the counter.
#[ic_cdk_macros::query]
fn get() -> Nat {
    COUNTER.with(|counter| (*counter.borrow()).clone())
}

/// Set the value of the counter.
#[ic_cdk_macros::update]
fn set(n: Nat) {
    // COUNTER.replace(n);  // requires #![feature(local_key_cell_methods)]
    COUNTER.with(|count| *count.borrow_mut() = n);
}

/// Increment the value of the counter.
#[ic_cdk_macros::update]
fn inc() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1_u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set() {
        let expected = Nat::from(42_u32);
        set(expected.clone());
        assert_eq!(get(), expected);
    }

    #[test]
    fn test_init() {
        assert_eq!(get(), Nat::from(0_u32));
    }

    #[test]
    fn test_inc() {
        for i in 1..10_u32 {
            inc();
            assert_eq!(get(), Nat::from(i));
        }
    }
}