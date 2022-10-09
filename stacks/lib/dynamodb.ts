import { Table, AttributeType, BillingMode, ProjectionType } from 'aws-cdk-lib/aws-dynamodb';
import { Stack } from 'aws-cdk-lib';

export const mainTable = (stack: Stack) => {
    const table = new Table(stack, 'mainTable', {
        partitionKey: { name: 'HK', type: AttributeType.STRING },
        sortKey: { name: 'SK', type: AttributeType.STRING },
        billingMode: BillingMode.PAY_PER_REQUEST,
        tableName: 'Main',
        timeToLiveAttribute: 'TTL',
    })

    table.addGlobalSecondaryIndex({
        indexName: 'GS1',
        partitionKey: {name: 'GS1HK', type: AttributeType.STRING},
        sortKey: {name: 'GS1SK', type: AttributeType.STRING },
        projectionType: ProjectionType.ALL
    })

    return table
}
