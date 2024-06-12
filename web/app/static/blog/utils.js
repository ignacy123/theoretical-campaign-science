let ad_directory = "/static/blog/ads/";

const ads = [
    {path: 'anvil.jpg', url: 'https://www.facebook.com/anvilkebab/', id: 'anvil'},
    {path: 'bidet.jpg', url: 'https://allegro.pl/listing?string=bidon', id: 'bidet'},
    // {path: 'bliblie_control.jpg', url: 'https://google.com', id: 'bliblie'},
    {path: 'burger.jpeg', url: 'https://forum.tcs.uj.edu.pl/t/burger/9526/4', id: 'burger'},
    {path: 'goloneczka.jpg', url: 'https://www.facebook.com/profile.php?id=100063181243376', id: 'goloneczka'},
    {
        path: 'micex.jpg',
        url: 'https://forum.tcs.uj.edu.pl/t/koszulka-jakby-znajoma-ale-nie-do-konca/1609/26',
        id: 'micex'
    },
    // {path: 'mints.jpg', url: 'https://google.com', id: 'mints'},
    // {path: 'nose_warmer.jpg', url: 'https://google.com', id: 'warmer'},
    // {path: 'oranzenada.jpg', url: 'https://google.com', id: 'oranzenada'},
    {path: 'pizza.jpg', url: 'https://en.wikipedia.org/wiki/None_Pizza_with_Left_Beef', id: 'pizza'},
    {
        path: 'programerki.jpg',
        url: 'https://www.reddit.com/r/linuxmasterrace/comments/shn1gz/what_socks_do_you_prefer_wearing_while_programming/',
        id: 'programerki'
    },
    {path: 'sciernisko.jpg', url: 'https://www.youtube.com/watch?v=54pA9uqh5AA', id: 'sciernisko'},
    {path: 'smalec.jpg', url: 'https://www.youtube.com/@CichyWuj/videos', id: 'smalec'},
    // {path: 'dupex.png', url: 'https://google.com', id: 'dupex'},
    {path: 'lung_extension.jpg', url: 'https://www.fbi.gov/investigate', id: 'lungs'},
    // {path: 'nosidelko.jpeg', url: 'https://google.com', id: 'carrier'},
    {
        path: 'probability.jpg',
        url: 'https://studiuje.uj.edu.pl/documents/139368252/147549725/instrukcja_wpis.warunkowy.pdf',
        id: 'probability'
    },
    {path: 'silksong.jpg', url: 'https://www.youtube.com/watch?v=ii02ZfT3Pec', id: 'silksong'},
    {path: 'car_0.jpg', url: 'https://tcs-cars.sytes.net/shop/', id: 'car_0'},
    {path: 'car_1.jpg', url: 'https://tcs-cars.sytes.net/shop/', id: 'car_1'},
    {path: 'car_2.jpg', url: 'https://tcs-cars.sytes.net/shop/', id: 'car_2'},
    {path: 'car_3.jpg', url: 'https://tcs-cars.sytes.net/shop/', id: 'car_3'},
    {path: 'car_4.jpg', url: 'https://tcs-cars.sytes.net/shop/', id: 'car_4'},
    {path: 'car_5.jpg', url: 'https://tcs-cars.sytes.net/shop/', id: 'car_5'},
    {path: 'car_6.jpg', url: 'https://tcs-cars.sytes.net/shop/', id: 'car_6'},
    {path: 'car_7.jpg', url: 'https://tcs-cars.sytes.net/shop/', id: 'car_7'},
    {path: 'car_8.jpg', url: 'https://tcs-cars.sytes.net/shop/', id: 'car_8'},
    {path: 'car_9.jpg', url: 'https://tcs-cars.sytes.net/shop/', id: 'car_9'},
    {path: 'car_10.jpg', url: 'https://tcs-cars.sytes.net/shop/', id: 'car_10'},
    {path: 'car_11.jpg', url: 'https://tcs-cars.sytes.net/shop/', id: 'car_11'},
    {path: 'car_12.jpg', url: 'https://tcs-cars.sytes.net/shop/', id: 'car_12'},
];

function getRandomAds(count) {
    let currentIndex = ads.length;

    while (currentIndex > ads.length - count) {
        let randomIndex = Math.floor(Math.random() * currentIndex);
        currentIndex -= 1;
        [ads[currentIndex], ads[randomIndex]] = [ads[randomIndex], ads[currentIndex]];
    }
    const result = ads.slice(ads.length - count, ads.length);
    return result;
}

function displayRandomAds() {
    const elements = [
        document.getElementById('ad1'),
        document.getElementById('ad2'),
        document.getElementById('ad3')
    ];
    const ads = getRandomAds(3);

    elements.forEach((element, index) => {
        const ad = ads[index];

        const a = document.createElement('a');
        a.href = './../ad_click?link=' + encodeURIComponent(ad.url) + '&id=' + encodeURIComponent(ad.id);
        a.target = '_blank';
        element.appendChild(a);

        const img = document.createElement('img');
        img.src = ad_directory + ad.path;
        img.alt = 'Ad ' + ad.id;
        a.appendChild(img);
    });
}

document.addEventListener('DOMContentLoaded', displayRandomAds);
