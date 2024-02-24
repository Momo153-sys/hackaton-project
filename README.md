# Icp Project

The code defines a system for managing tasks, where the user can add or remove tasks and display them. It also allows the user to make a request from Open Weather API to get the Istanbul weather information from it and depending on the temperature make or not an nft transfer. It uses the Internet Computer's capabilities to make HTTP requests.

### Key Structures

1. **Nft**

    - `owner`: A `String` representing the name of the nft owner.
    - `id`: A `Number` representing the id of the nft owner.

2. **Task**

    - `tasks`: A `Vec<String>` storing the tasks.

3. **check**
    - A module which contains `CONDITION` which controls whether or not the nft transfer condition is met and `TASKS` which keep track of all the changes made to the task list.

### Nft Implementations

-   Both `new` and `transfer` structures implement the `Nft` struct , allowing them to create a new `Nft` instance and make transfer.

### Task Implementations

-   All `new` and `add_task` and `erase_task` structures implement the `Task` struct , allowing them to create a new `Task` instance and make changes to the task list.

### Functions

1. **display_data(api_key:String)**

    - Take the APi key as parameter and make Http request and return the result either the information or the error as a string.

2. **transform(raw: TransformArgs)**

    - Intervene in the http request process and is essential for a positive outcome.

3. **nft_transfer(receiver:String)**

    - Make the nft transfer and display the message as an output.

4. **initialize_list()**

    - Create an initialized task list.

5. **add_task(description:String)**

    - Take a task description as parameter and create a new task which will be added to the list.

6. **erase_task(description:String)**

    - Take task description as a parameter and check if it exists in the list then erase it. If it does not exist an error message is returned.

7. **display_list()**
    - Display the current task list.

7. **get()**
    - Get the value of the counter.

7. **set()**
     - Set the value of the counter.

7. **inc()**
    - Increment the value of the counter.   

### HTTP Request Handling

-   The `display_data` function demonstrates how to make an HTTP GET request, handle the response, and update the stored events.



## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background --clean

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.
