```gql
{
  employees {
    id
    details {
      forename
      surname
      location
    }
    role {
      title
      departments
      ... on Engineer {
        engineerType
      }
      ... on Operator {
        operatorType
      }
    }
    startDate
    notes
    tag
    updatedAt
    __typename
  }
}
```

```gql
{
  products {
    __typename
    ... on Cosmo {
      upc
      lead {
        id
      }
      engineers {
        id
      }
    }
    ... on Consultancy {
      upc
      lead {
        id
      }
    }
    ... on SDK {
      upc
      owner {
        id
      }
      engineers {
        id
      }
    }
  }
}
```

```gql
{
  teammates(team:ENGINEERING) {
    id
  }
}
```