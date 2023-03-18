Prepare dependencies: ```cargo chef prepare --recipe-path recipe.json```

Build image: ```docker image build . -t registry.digitalocean.com/bec-registry/bec_backend:0.0.1```

Push image: ```docker push registry.digitalocean.com/bec-registry/bec_backend:0.0.1```

Run image (local): ```docker run -p 8000:8000 -d registry.digitalocean.com/bec-registry/bec_backend:0.0.1```

Deploy: ```kubectl deploy k8s/deployment.yaml```