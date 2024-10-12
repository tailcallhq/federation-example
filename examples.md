```gql
query Employees {
    employees {
        currentMood
        id
        notes
        products
        startDate
        tag
        updatedAt
        details {
            forename
            hasChildren
            maritalStatus
            middlename
            nationality
            surname
            pets {
                class
                gender
                name
                ... on Alligator {
                    class
                    gender
                    name
                    dangerous
                }
                ... on Cat {
                    class
                    gender
                    name
                    type
                }
                ... on Dog {
                    breed
                    class
                    gender
                    name
                }
                ... on Mouse {
                    class
                    gender
                    name
                }
                ... on Pony {
                    class
                    gender
                    name
                }
            }
        }
        role {
            departments
            title
            ... on Engineer {
                departments
                engineerType
                title
            }
            ... on Marketer {
                departments
                title
            }
            ... on Operator {
                departments
                operatorType
                title
            }
        }
    }
    factTypes
    productTypes {
        ... on Consultancy {
            name
            upc
            lead {
                id
                startDate
                tag
                updatedAt
            }
        }
        ... on Cosmo {
            name
            repositoryUrl
            upc
        }
        ... on Documentation {
            url(product: "Test")
            urls(products: "Test")
        }
        ... on SDK {
            upc
            owner {
                id
                startDate
                tag
                updatedAt
            }
            engineers {
                id
                updatedAt
            }
        }
    }
    products {
        ... on Consultancy {
            name
            upc
        }
        ... on Cosmo {
            name
            repositoryUrl
            upc
        }
        ... on SDK {
            upc
        }
    }
    teammates(team: "Engineering") {
        id
        notes
        products
        startDate
        tag
        updatedAt
        details {
            forename
            hasChildren
            maritalStatus
            middlename
            nationality
            surname
            pets {
                class
                gender
                name
            }
        }
    }
    topSecretFederationFacts {
        description
        factType
        ... on DIRECTIVEFact {
            _DIRECTIVEFact
            description
            factType
            title
        }
        ... on EntityFact {
            _entityFact
            description
            factType
            title
        }
        ... on MiscellaneousFact {
            _miscellaneousFact
            description
            factType
            title
        }
    }
}

```