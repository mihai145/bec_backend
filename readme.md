# Team bec
### Members
<li>Airinei Andrei Cristian (gr 252) </li>
<li>Dimitriu Andrei Dragos (gr 252) </li>
<li>Gheorge Liviu Armand (gr 252) </li>
<li>Gheorge Marius Catalin (gr 251) </li>
<li>Oprea Mihai Adrian (gr 252) </li>

### Epic stories
 https://docs.google.com/document/d/1ABb4r_hdL_LdkEk6mRNq0DWHuSjYFZ-vMU_UJOMtrPE/edit?usp=sharing

### User stories
 https://docs.google.com/document/d/1FQGcLv-_Ce2IIifc6E2UAafGtPCO9GO_mnbch_Fdfmc/edit?usp=sharing

### Backlog
 https://trello.com/invite/b/j2L9dYsv/ATTIf0a5d39020a8d9751abf85adb7a40ad251B5AF4F/bec-film-app

#### Diagrama
 ![Diagrama](mds-diagrama.jpeg)

#### Demo
https://drive.google.com/file/d/1QQfYWVcuEYvx3iyTuMBQN0iMvNZaaBW4/view?usp=sharing

# Architecture
![Architecture](mds-architecture.jpg)

# Commands
Prepare dependencies: ```cargo chef prepare --recipe-path recipe.json```

Build image: ```docker image build . -t registry.digitalocean.com/bec-registry-2/bec_backend:0.0.1```

Push image: ```docker push registry.digitalocean.com/bec-registry-2/bec_backend:0.0.1```

Run image (local): ```docker run -p 8000:8000 -d registry.digitalocean.com/bec-registry-2/bec_backend:0.0.1```

Deploy: ```kubectl apply -f k8s/deployment.yaml```
