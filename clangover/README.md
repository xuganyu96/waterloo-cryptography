TODO: need to fix attack output

```bash
scp -i $PEM_PATH -r ../clangover ${REMOTE_USER}@${REMOTE_HOST}:/home/${REMOTE_USER}/
docker build -t clangover:latest -f clangover.Dockerfile .
docker run --rm -d --name "clangover" clangover:latest
docker logs -f clangover
```
