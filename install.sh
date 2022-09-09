#!/bin/bash

cargo build --release

sudo rm -rf /usr/local/lib/expenses
sudo rm -rf /usr/local/bin/expenses

sudo mkdir /usr/local/lib/expenses/

sudo mv target/release/* /usr/local/lib/expenses

sudo ln -s /usr/local/lib/expenses/main /usr/local/bin/expenses