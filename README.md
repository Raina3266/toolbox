## Tasks

New endpoints (i.e. handlers)

 - "/create-user" - accepts a post request containing a username and password, and creates a user with that username and password, with a random user id. It then also logs the user in, and returns a session token to them
 - "/logout" - accepts a post request containing a session token and deletes the session