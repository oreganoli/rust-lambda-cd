# rust-lambda-cd
A simple Lambda updater in Rust, shamelessly aped from [an AWS tutorial](https://aws.amazon.com/blogs/compute/new-deployment-options-for-aws-lambda/).
## Usage
Set up your CI/CD service so that for an AWS Lambda function named *function_X*, a file `function_X.zip` containing its code is uploaded to an S3 bucket, optionally to a subdirectory.
Set environment variables as follows:
```shell script
AWS_ACCESS_KEY_ID="YOUR ID"
AWS_SECRET_ACCESS_KEY="YOUR KEY"
AWS_REGION="your-region-x"
FUNCTION_NAMES="one_lambda:another:a_third_one" # if left blank, all new .zips in DIRECTORY will be checked and uploaded if they match an existent function's name
DIRECTORY="directory/within_your_bucket" # if left blank, all new .zips will be checked regardless of where they are within the bucket
```
Give this lambda full S3 and Lambda permissions. Hook it up to file creation events on your bucket.