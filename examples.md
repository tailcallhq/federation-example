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

```gql
{
  findEmployees {
    id
    details {
      forename
      surname
      middlename
      hasChildren
      maritalStatus
      nationality
      pets {
        name
        gender
        class
      }
    }
  }
}
```

```gql
{
  _consultancy_resolver(upcs: ["consultancy"]) {
    upc
  }
  _cosmo_resolver(upcs: ["cosmo"]) {
    upc
  }
  _sdk_resolver(upcs: ["sdk"]) {
    upc
  }
}
```