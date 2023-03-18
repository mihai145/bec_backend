Prepare dependencies: ```cargo chef prepare --recipe-path recipe.json```

Build image: ```docker image build . -t bec_backend:0.0.1```

Run image: ```docker run -p 8000:8000 -d bec_backend:0.0.1```