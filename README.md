## Proxy-Server with caching capability 

Project Board: https://github.com/users/StanDarius24/projects/1

This is a proof of concept.
==========================

The application serves as a middleware service capable of receiving HTTP requests, decoding them, 
and performing various actions, such as invoking other services with different types of authorization. 
It acts as a 'man in the middle,' providing an additional layer of security by preventing the direct 
injection of secrets into microservices. The service is designed to operate within the intranet 
as a 'hidden service' within the infrastructure.
Additionally, the application implements a caching mechanism with two layers, utilizing redis (l1) and mongoDB (l2) 
to enhance performance and efficiency.
