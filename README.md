# url shortener service

### Core Functionalities
- Users can submit a long URL and receive a shortened version.

- When accessing a shortened URL, users are redirected to the original URL.

- The redirection should be blazing fast, with minimal latency.

- Users can have many short urls, users for now will not be authenticated.

### Approach: 
use nanoId to generate unique short codes i.e, IDs 

#### Advantages:
- Simple implementation, 
- Fast lookup given the short code

#### Disadvantages:

- Fixed lenght of code: For instance for 8 characters, and N=64 alphabet, we can generate ~281 trillions unique codes (64^8) and after genrating ~17 million codes, there is a ~1% chance of collision. more info [here](https://zelark.github.io/nano-id-cc/)


#### Future Considerations:
- add expiration time for the short codes and remove them from the database
- or use another approach to generate unique codes.


### How to run locally
#### requirements: 
- rust installed locally : [install rust](https://www.rust-lang.org/tools/install)
- docker and docker-compose: [install docker](https://docs.docker.com/get-docker/)
#### run the project:
- clone the repository
- set the .env for local dev, check the .env.example 
- ```Make init``` to setup the project
- ```Make run``` to run the project
- ```Make test``` to run the tests

