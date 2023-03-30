Prepare dependencies: ```cargo chef prepare --recipe-path recipe.json```

Build image: ```docker image build . -t registry.digitalocean.com/bec-registry/bec_backend:0.0.1```

Push image: ```docker push registry.digitalocean.com/bec-registry/bec_backend:0.0.1```

Run image (local): ```docker run -p 8000:8000 -d registry.digitalocean.com/bec-registry/bec_backend:0.0.1```

Deploy: ```kubectl apply -f k8s/deployment.yaml```

<br>
## Endpoints (local)

Lista de filme dupa nume:
```
POST localhost:8000/search/movieName 
{
    "movieName": "avatar"
}
```
<br>

Informatii despre film dupa id:
```
POST localhost:8000/search/movieId
{
    "movieId": 76600
}
```
<br>

Lista de actori dupa nume:
```
POST localhost:8000/search/actorName
{
    "actorName": "Jason Stat"
}
```
<br>

Informatii despre actor dupa id:
```
POST localhost:8000/search/actorId
{
    "actorId": 23
}
```
<br>

Lista de id-uri pentru genres:
<br>
```GET localhost:8000/genres```
<br>

Filme trending in saptamana curenta dupa genre:
```
POST localhost:8000/trending
{
    "genreId": 28
}
```