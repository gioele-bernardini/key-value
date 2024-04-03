#!/usr/bin/bash

# Create a dummy username and password that you can verify against
SAVED_USERNAME="johndoe"
SAVED_PASSWORD="password"

read -p "Enter your username: " USERNAME

if [[ -z $USERNAME ]]
then 
  echo "Error: Username cannot be empty"
  exit 1
fi

read -sp "Enter password: " PASSWORD
echo

if [[ -z $PASSWORD ]]
then
  echo "Error: Password cannot be empty"
  exit 1
fi

if [[ $USERNAME == $SAVED_USERNAME && $PASSWORD == $SAVED_PASSWORD ]]
then
  echo "Logged in successfully!"
elif [[ $USERNAME == $SAVED_USERNAME && $PASSWORD != $SAVED_PASSWORD || $PASSWORD == $SAVED_PASSWORD && $USERNAME != $SAVED_USERNAME ]]
then
  echo "Invalid credentials, Username and Password did not match"
fi

exit 0

