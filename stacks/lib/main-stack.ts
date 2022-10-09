import { Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { mainTable } from './dynamodb';

export class MainStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);
    mainTable(this)
  }
}
