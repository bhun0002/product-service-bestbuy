use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Hi-Tech Earphones".to_string(),
            price: 9.99,
            description: "Experience superior sound quality with Hi-Tech Earphones. These wireless earphones feature active noise cancellation, long battery life, and fast charging capabilities. Enjoy crystal-clear audio and an ergonomic fit, along with a built-in voice assistant for hands-free control.".to_string(),
            image: "/hitech.png".to_string()
        },
        Product {
            id: 2,
            name: "Hi-tech Charger".to_string(),
            price: 6.99,
            description: "Experience efficient charging with the advanced wireless charger. It features fast charging capabilities, USB-C compatibility, and supports multi-device charging. Its compact design and portability make it ideal for on-the-go power needs.".to_string(),
            image: "/charger.png".to_string()
        },
        Product {
            id: 3,
            name: "HP 15.6' Laptop - Natural silver".to_string(),
            price: 12.99,
            description: "Enhance your productivity with the HP 15.6' Laptop in Natural Silver. It features a Full HD display, Intel Core processor, SSD storage, and Iris Xe Graphics, all within a lightweight design that offers all-day battery life and runs on Windows 11.".to_string(),
            image: "/hp.png".to_string()
        },
    ]
}