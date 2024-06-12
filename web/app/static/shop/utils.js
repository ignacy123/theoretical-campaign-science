let car_directory = "/static/shop/cars/";

const cars = [
    'car_0.jpg',
    'car_1.jpg',
    'car_2.jpg',
    'car_3.jpg',
    'car_4.jpg',
    'car_5.jpg',
    'car_6.jpg',
    'car_7.jpg',
    'car_8.jpg',
    'car_9.jpg',
    'car_10.jpg',
    'car_11.jpg',
    'car_12.jpg',
];

function displayRandomCar() {
    const car = cars[Math.floor(Math.random() * cars.length)];

    console.log(car);
    const element = document.getElementById('car')

    const a = document.createElement('a');
    element.appendChild(a);

    const img = document.createElement('img');
    img.src = car_directory + car;
    img.alt = 'A car image.';
    a.appendChild(img);
}

document.addEventListener('DOMContentLoaded', displayRandomCar);
