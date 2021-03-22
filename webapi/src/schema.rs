table! {
    customers (customerNumber) {
        customerNumber -> Integer,
        customerName -> Varchar,
        contactLastName -> Varchar,
        contactFirstName -> Varchar,
        phone -> Varchar,
        addressLine1 -> Varchar,
        addressLine2 -> Nullable<Varchar>,
        city -> Varchar,
        state -> Nullable<Varchar>,
        postalCode -> Nullable<Varchar>,
        country -> Varchar,
        salesRepEmployeeNumber -> Nullable<Integer>,
        // creditLimit -> Nullable<Decimal>, // cant have no decimal
    }
}

table! {
    employees (employeeNumber) {
        employeeNumber -> Integer,
        lastName -> Varchar,
        firstName -> Varchar,
        extension -> Varchar,
        email -> Varchar,
        officeCode -> Varchar,
        reportsTo -> Nullable<Integer>,
        jobTitle -> Varchar,
    }
}

table! {
    offices (officeCode) {
        officeCode -> Varchar,
        city -> Varchar,
        phone -> Varchar,
        addressLine1 -> Varchar,
        addressLine2 -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        country -> Varchar,
        postalCode -> Varchar,
        territory -> Varchar,
    }
}

table! {
    orderdetails (orderNumber, productCode) {
        orderNumber -> Integer,
        productCode -> Varchar,
        quantityOrdered -> Integer,
        priceEach -> Decimal,
        orderLineNumber -> Smallint,
    }
}

table! {
    orders (orderNumber) {
        orderNumber -> Integer,
        orderDate -> Date,
        requiredDate -> Date,
        shippedDate -> Nullable<Date>,
        status -> Varchar,
        comments -> Nullable<Text>,
        customerNumber -> Integer,
    }
}

table! {
    payments (customerNumber, checkNumber) {
        customerNumber -> Integer,
        checkNumber -> Varchar,
        paymentDate -> Date,
        amount -> Decimal,
    }
}

table! {
    productlines (productLine) {
        productLine -> Varchar,
        textDescription -> Nullable<Varchar>,
        htmlDescription -> Nullable<Mediumtext>,
        image -> Nullable<Mediumblob>,
    }
}

table! {
    products (productCode) {
        productCode -> Varchar,
        productName -> Varchar,
        productLine -> Varchar,
        productScale -> Varchar,
        productVendor -> Varchar,
        productDescription -> Text,
        quantityInStock -> Smallint,
        buyPrice -> Decimal,
        MSRP -> Decimal,
    }
}

table! {
    users (user_name) {
        id -> Integer,
        create_time -> Nullable<Varchar>,
        update_time -> Nullable<Varchar>,
        user_name -> Varchar,
        user_password -> Varchar,
        user_email -> Varchar,
    }
}

joinable!(customers -> employees (salesRepEmployeeNumber));
joinable!(employees -> offices (officeCode));
joinable!(orderdetails -> orders (orderNumber));
joinable!(orderdetails -> products (productCode));
joinable!(orders -> customers (customerNumber));
joinable!(payments -> customers (customerNumber));
joinable!(products -> productlines (productLine));

allow_tables_to_appear_in_same_query!(
    customers,
    employees,
    offices,
    orderdetails,
    orders,
    payments,
    productlines,
    products,
    users,
);
