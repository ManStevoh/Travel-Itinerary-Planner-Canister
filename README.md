### Travel Itinerary Planner Deployment Guide

#### Overview:

This Rust code constitutes a travel itinerary planner designed for deployment on the Internet Computer (IC) platform. The application manages travel plans, accommodations, transportation bookings, and budgeting for a trip. Here's an overview of the deployment process:

#### Prerequisites:

1. **Rust Installation:**
   - Ensure Rust is installed using [rustup](https://rustup.rs/).

2. **Internet Computer SDK:**
   - Install the DFINITY Canister SDK to interact with the Internet Computer locally. Follow [official documentation](https://sdk.dfinity.org/docs/quickstart/local-quickstart.html).

#### Deployment Steps:

1. **Clone the Repository:**
   - Clone the repository from the provided URL and navigate to the project directory.
     ```bash
     git clone https://github.com/ManStevoh/Travel-Itinerary-Planner-Canister.git
     cd Travel-Itinerary-Planner-Canister
     ```

2. **Build the Code:**
   - Build the Rust code using the Cargo package manager.
     ```bash
     cargo build
     ```

3. **Run the Local Internet Computer Replica:**
   - Start the local replica, simulating the IC network.
     ```bash
     dfx start
     ```

4. **Deploy the Canister:**
   - Create and install the canister locally using the DFINITY command-line tool.
     ```bash
     dfx canister create --all
     dfx canister install --all
     ```

5. **Interact with the Canister:**
   - Utilize the deployed canister through the provided Candid interface. Execute queries or updates using the defined functions.

6. **Test the Functions:**
   - Test functions using the `dfx` command-line tool. Example:
     ```bash
     dfx canister call <canister-name> get_travel_plan 1
     ```

7. **Stop the Local Replica:**
   - Terminate the local replica when finished testing.
     ```bash
     dfx stop
     ```
