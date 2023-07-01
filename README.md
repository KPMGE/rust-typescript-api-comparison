# rust-typescript-api-comparison

## Dependencies
Before running this project, make sure you got docker, docker-compose, rust, sqlx, prisma and nodejs properly
installed and configured. 

## How do i run the apis? 
Before running the apis, make sure you got a database instance running, you can
do that using docker. At the root directory, run the command: 

```bash
sudo docker-compose up -d
```

Afer a few seconds you should have a postgres database instance up and running.
Now you can navigate to one of the api folders, there will be more information
about how to execute them in there.

## Run comparison
if you want to run the comparison and see the results, it's really simple,
assuming you got python on your machine, start up one of the apis and then run 
the command: 

```bash
python3 compare.py
```

## Api routes
You can have access to all routes in the _documentation_ folder
