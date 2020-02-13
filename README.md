# rust-lambda-cd
A simple Lambda updater in Rust, shamelessly aped from [an AWS tutorial](https://aws.amazon.com/blogs/compute/new-deployment-options-for-aws-lambda/).
## Usage
Set up your CI/CD service so that for an AWS Lambda function named *function_X*, a file `function_X.zip` containing its code is uploaded to an S3 bucket.
Set environment variables as follows:
```shell script
AWS_ACCESS_KEY_ID="YOUR ID"
AWS_SECRET_ACCESS_KEY="YOUR KEY"
AWS_REGION="your-region-x"
```
Upload this lambda and give it the appropriate S3 and Lambda permissions. Hook it up to file creation events on your bucket with a ".zip" suffix and optionally a prefix corresponding to the directory your build artifacts are in.
If you would like to only update a few chosen lambdas, you may additionally set `FUNCTION_NAMES="one_lambda:another:a_third_one"`.