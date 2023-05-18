## HTTP Pub/Sub Publisher

<div> 
  <p>This application parses a HTTP Request and publish it to a Pub/Sub topic</p> 
</div>

<p>Motivation: Use Pub/Sub queues from third party applications</p>

<h4>Authentication and IAM</h4>

<p>The current auth method pulls the authentication natively from the Service Account running the Cloud Run container</p>
<p>Check the 
  <a href=https://docs.rs/google-cloud-pubsub/latest/google_cloud_pubsub> Pub/Sub Library </a>
for other auth methods</p>

<h4>Deploying to Google Cloud Run</h4>

```sh
# Use image from elsewhere
gcloud run deploy <<container-name>> --source=<<image>>  --port=8080
```

```sh
# Auto build docker image
gcloud run deploy <<container-name>> --source=.  --port=8080
```

<h4>Making a request</h4>

```sh
POST /:topic -> Your Pub/Sub topic name
Body -> Pub/Sub message data
Headers -> Pub/Sub message attributes
```
