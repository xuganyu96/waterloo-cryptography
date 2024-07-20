TODO: need to fix attack output

```bash
export PEM_PATH="/path/to/pem"
export REMOTE_USER="ec2-user"
export REMOTE_HOST="aws.amazon.com"
scp -i $PEM_PATH -r ../clangover ${REMOTE_USER}@${REMOTE_HOST}:/home/${REMOTE_USER}/
ssh -i $PEM_PATH $REMOTE_USER@$REMOTE_HOST
docker build -t clangover:latest -f clangover.Dockerfile .
docker run --rm -d --name "clangover" clangover:latest
docker logs -f clangover
```
