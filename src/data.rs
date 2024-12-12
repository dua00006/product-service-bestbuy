use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Samsung 55\" 4K Smart TV".to_string(),
            price: 499.99,
            description: "Experience stunning 4K UHD resolution and smart streaming capabilities with Samsung's 55-inch Smart TV.".to_string(),
            image: "/samsung_tv.jpg".to_string(),
        },
        Product {
            id: 2,
            name: "Apple MacBook Pro 14\" M2".to_string(),
            price: 1999.99,
            description: "The latest Apple MacBook Pro with the M2 chip offers powerful performance and a stunning Retina display.".to_string(),
            image: "/macbook_pro.jpg".to_string(),
        },
        Product {
            id: 3,
            name: "Bose QuietComfort 45 Headphones".to_string(),
            price: 329.99,
            description: "Immerse yourself in music with Bose's noise-canceling QuietComfort 45 headphones, perfect for travel and daily use.".to_string(),
            image: "/bose_headphones.jpg".to_string(),
        },
        Product {
            id: 4,
            name: "Sony PlayStation 5 Console".to_string(),
            price: 499.99,
            description: "Get the ultimate gaming experience with Sony's PlayStation 5, featuring ultra-fast loading and stunning graphics.".to_string(),
            image: "/ps5.jpg".to_string(),
        },
        Product {
            id: 5,
            name: "Dyson V15 Detect Vacuum".to_string(),
            price: 749.99,
            description: "Keep your home clean with the Dyson V15 Detect Vacuum, featuring laser detection and powerful suction.".to_string(),
            image: "/dyson_vacuum.jpg".to_string(),
        },
        Product {
            id: 6,
            name: "KitchenAid Artisan Stand Mixer".to_string(),
            price: 449.99,
            description: "Enhance your baking with the KitchenAid Artisan Stand Mixer, a versatile and durable kitchen companion.".to_string(),
            image: "/kitchenaid_mixer.jpg".to_string(),
        },
        Product {
            id: 7,
            name: "Ring Video Doorbell 4".to_string(),
            price: 199.99,
            description: "Monitor your home and answer your door remotely with the Ring Video Doorbell 4, featuring HD video and motion detection.".to_string(),
            image: "/ring_doorbell.jpg".to_string(),
        },
        Product {
            id: 8,
            name: "Canon EOS R6 Mirrorless Camera".to_string(),
            price: 2499.99,
            description: "Capture breathtaking photos and videos with the Canon EOS R6, offering exceptional image quality and versatility.".to_string(),
            image: "/canon_camera.jpg".to_string(),
        },
        Product {
            id: 9,
            name: "Fitbit Charge 5 Fitness Tracker".to_string(),
            price: 179.99,
            description: "Track your health and fitness goals with the Fitbit Charge 5, featuring heart rate monitoring and built-in GPS.".to_string(),
            image: "/fitbit_charge5.jpg".to_string(),
        },
        Product {
            id: 10,
            name: "Ninja Foodi Air Fryer".to_string(),
            price: 149.99,
            description: "Cook healthier meals with the Ninja Foodi Air Fryer, offering multiple cooking functions and rapid air technology.".to_string(),
            image: "/ninja_airfryer.jpg".to_string(),
        },
    ]
}

