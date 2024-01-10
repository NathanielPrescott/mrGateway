## What is mrGateway?
_______________

The goal of mrGateway (Microservice Rudimentary Gateway) is to allow for fast setup and integration with a microservice built to handle gRPC/REST requests of other services.
This service will connect with mrCache to give the ability to cache responses from other services in a single place if caching doesn't exist already for your application.
This service will take in a request and forward it to the appropriate service and return the response to the requester.

## Building
_______________

I have built with the following versions.

    Rust: 1.74.0
    Docker: 24.0.7

I have currently setup and tested mrGateway running in a docker container.

    docker build -t mrgateway .
    docker run --name mrGateway -p 8080:8080 mrgateway

## Future Features
_______________

This is based on community response and my own ideas. I'm open to suggestions and PRs.