//! This module contains icons from font awesome. Instead of using the svg
//! files, we embed the paths directly (because that makes it easier to change
//! their colors). Some of these images (specifically CAMERA, VIDEO,
//! MICROPHONE, PLAY, PAUSE, FADE_OUT) were derived from font-awesome, which is
//! available under a CC-BY license.
//! (TODO: is this comment enough for the attribution requirements?)

/// An icon made up of a single path (which should be filled with whatever color we want).
pub struct Icon {
    /// The width of the icon.
    pub width: u32,
    /// The height of the icon.
    pub height: u32,
    /// The icon's path, in SVG format.
    pub path: &'static str,
}

#[allow(dead_code)]
pub const CAMERA: Icon = Icon {
    width: 512,
    height: 512,
    path: "M512 144v288c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V144c0-26.5 21.5-48 48-48h88l12.3-32.9c7-18.7 24.9-31.1 44.9-31.1h125.5c20 0 37.9 12.4 44.9 31.1L376 96h88c26.5 0 48 21.5 48 48zM376 288c0-66.2-53.8-120-120-120s-120 53.8-120 120 53.8 120 120 120 120-53.8 120-120zm-32 0c0 48.5-39.5 88-88 88s-88-39.5-88-88 39.5-88 88-88 88 39.5 88 88z",
};

pub const VIDEO: Icon = Icon {
    width: 576,
    height: 512,
    path: "M336.2 64H47.8C21.4 64 0 85.4 0 111.8v288.4C0 426.6 21.4 448 47.8 448h288.4c26.4 0 47.8-21.4 47.8-47.8V111.8c0-26.4-21.4-47.8-47.8-47.8zm189.4 37.7L416 177.3v157.4l109.6 75.5c21.2 14.6 50.4-.3 50.4-25.8V127.5c0-25.4-29.1-40.4-50.4-25.8z",
};

pub const MICROPHONE: Icon = Icon {
    width: 352,
    height: 512,
    path: "M176 352c53.02 0 96-42.98 96-96V96c0-53.02-42.98-96-96-96S80 42.98 80 96v160c0 53.02 42.98 96 96 96zm160-160h-16c-8.84 0-16 7.16-16 16v48c0 74.8-64.49 134.82-140.79 127.38C96.71 376.89 48 317.11 48 250.3V208c0-8.84-7.16-16-16-16H16c-8.84 0-16 7.16-16 16v40.16c0 89.64 63.97 169.55 152 181.69V464H96c-8.84 0-16 7.16-16 16v16c0 8.84 7.16 16 16 16h160c8.84 0 16-7.16 16-16v-16c0-8.84-7.16-16-16-16h-56v-33.77C285.71 418.47 352 344.9 352 256v-48c0-8.84-7.16-16-16-16z",
};

pub const PLAY: Icon = Icon {
    width: 448,
    height: 512,
    path: "M424.4 214.7L72.4 6.6C43.8-10.3 0 6.1 0 47.9V464c0 37.5 40.7 60.1 72.4 41.3l352-208c31.4-18.5 31.5-64.1 0-82.6z",
};

pub const PAUSE: Icon = Icon {
    width: 448,
    height: 512,
    path: "M144 479H48c-26.5 0-48-21.5-48-48V79c0-26.5 21.5-48 48-48h96c26.5 0 48 21.5 48 48v352c0 26.5-21.5 48-48 48zm304-48V79c0-26.5-21.5-48-48-48h-96c-26.5 0-48 21.5-48 48v352c0 26.5 21.5 48 48 48h96c26.5 0 48-21.5 48-48z",
};

pub const SNAIL: Icon = Icon {
    width: 148,
    height: 135,
    path: "M34.89 133.9 C13.89 131.88 0.41 127.02 0.41 121.46 c0 -4.34 5.62 -5.21 17.13 -2.65 13.31 2.96 28.37 2.04 38.27 -2.34 3.61 -1.6 7.83 -2.9 9.39 -2.9 8.65 0 16.83 -3.71 24 -10.89 8.71 -8.73 11.86 -17.38 10.32 -28.33 -2.68 -19.05 -2.7 -20.99 -0.21 -27.5 4.27 -11.19 14.07 -16 25.27 -12.42 9.42 3.02 16.59 11.55 16.59 19.74 0 5.05 -5.15 14.92 -8.8 16.88 -2.11 1.13 -2.93 3.09 -2.95 7.01 -0.02 3 -0.95 9.83 -2.09 15.19 -7.53 35.5 -32.34 46.42 -92.44 40.65zM22.71 113.87 c-7.1 -2.19 -16.14 -21.64 -16.14 -34.24 0 -14.39 10.62 -29.99 25.37 -36.71 10.24 -4.66 26.42 -4.66 36.66 0 13.55 6.18 23.33 18.47 25.13 31.6 1.9 13.86 -7.51 27.71 -21.56 31.74l-4.09 1.17 2.83 -4.63c4.06 -6.67 3.82 -19.18 -0.54 -27.42 -7.96 -15.05 -30.71 -17.61 -36.32 -4.08 -3.72 8.97 3.19 21.03 11.43 19.98 4.66 -0.59 4.57 -2.96 -0.22 -5.32 -5.37 -2.65 -7.68 -7.95 -5.53 -12.67 2.17 -4.77 8.16 -6.15 15.45 -3.57 8.47 3 11.96 8.47 11.96 18.77 0 10.37 -4.48 17.88 -13.2 22.09 -6.28 3.04 -25.48 5.06 -31.22 3.29zm86.88 -91.81c-2.16 -14.41 5.37 -26.5 12.23 -19.64 2.99 2.99 1.95 6.86 -2.45 9.14 -4.32 2.23 -6 5.83 -6.03 12.93 -0.03 6.4 -2.68 4.68 -3.74 -2.43zm15.45 4.95c0 -3.42 7.6 -16.19 10.94 -18.37 4.27 -2.8 9.06 -1.67 10.37 2.44 1.26 3.98 -1.74 7.19 -6.72 7.19 -4.47 0 -8 2.74 -9.46 7.34 -0.95 2.99 -5.13 4.14 -5.13 1.41z",
};

pub const RABBIT: Icon = Icon {
    width: 135,
    height: 135,
    path: "M29.154 132.286c-1.994-.231-4.683-1.589-5.976-3.017-2.063-2.28-2.15-3.433-.706-9.449.904-3.768 1.245-7.498.757-8.287-1.59-2.573-3.58-1.518-5.041 2.675-1.114 3.195-2.243 4.101-5.067 4.067-5.847-.07-11.17-5.794-11.17-12.013 0-3.506 5.027-9.761 9.086-11.304 2.521-.959 3.245-2.257 3.245-5.82 0-15.934 14.416-32.753 31.514-36.769 2.638-.62 9.514-.748 15.28-.286 10.15.813 10.51.963 11.26 4.71.928 4.644 9.337 14.292 15.015 17.228 2.245 1.161 8.92 2.442 14.832 2.848 5.913.405 10.751 1.204 10.751 1.775s-1.471 4.215-3.27 8.097l-3.27 7.059 2.408 9.364c3.031 11.782 3.126 11.92 8.853 12.995 5.348 1.003 10.351 6.37 10.351 11.104 0 3.939-2.259 4.758-13.129 4.758h-8.794v-4.179c0-8.706-5.63-14.753-15.756-16.926-4-.857-4.1-1.075-3.712-8.043.721-12.937-8.557-21.45-23.45-21.518-5.565-.026-6.536.348-6.147 2.367.339 1.761 2.301 2.623 7.39 3.246 3.812.467 8.242 1.55 9.846 2.409 7.774 4.16 9.591 14.615 3.622 20.845l-3.544 3.699 9.934 1.675c10.537 1.778 16.337 5.581 16.337 10.714 0 5.509-2.667 5.968-35.926 6.186-17.543.115-33.529.02-35.523-.21zM93.926 69.77C83.66 66.416 75.94 56.818 75.94 47.415c0-4.263.436-4.015-21.238-12.092-12.388-4.616-17.127-8.24-17.127-13.097 0-1.44 1.49-3.297 3.312-4.126 4.515-2.058 19.119 2.476 30.818 9.567l8.781 5.323 4.716-3.2c13.122-8.905 30.506-4.438 41.433 10.646 8.704 12.015 8.567 22.906-.346 27.532-6.776 3.516-24.157 4.484-32.363 1.803zM113.85 53.13c.456-1.189-.124-3.23-1.413-4.404-1.125-1.025-3.628-.79-4.486 0-2.801 2.801-1.515 6.565 2.243 6.565 1.554 0 3.2-.972 3.656-2.161zM74.52 24.235c-3.364-1.769-10.37-5.223-15.568-7.676-10.79-5.092-13.292-7.813-10.393-11.306 3.901-4.7 22.047 1.568 35.272 12.184l5.835 4.685-4.514 2.664-4.515 2.664z",
};

pub const TURTLE: Icon = Icon {
    width: 180,
    height: 135,
    path: "M18.678 133.248c-3.255-1.39-2.73-6.456 1.345-12.961 1.958-3.128 3.566-6.405 3.573-7.283.006-.878-5.018-1.304-11.165-.948-9.36.543-11.27.129-11.76-2.55-.322-1.762 1.322-4.518 3.664-6.14 3.91-2.709 4.81-2.632 11.312.971 5.853 3.244 9.1 3.73 18.932 2.834 26.585-2.423 69.392-20.42 83.842-35.25 7.726-7.93 10.48-16.711 7.713-24.601-3.341-9.53 8.204-35.718 17.552-39.812 20.073-8.794 43.65 22.164 33.184 43.57-5.114 10.458-16.98 14.604-26.453 9.243-3.271-1.851-4.401-1.389-8.104 3.317-7.244 9.205-12.916 23.733-12.935 33.127-.016 7.68 2.465 19.186 6.143 28.49 2.213 5.596-3.713 9.248-15.006 9.248-8.828 0-11.09-.748-15.108-4.997-2.599-2.749-4.725-6.2-4.725-7.668 0-2.09-1.46-2.343-6.722-1.163-3.697.829-14.764 1.479-24.593 1.444l-17.872-.064-5.167 6.224c-4.736 5.705-6.013 6.212-15.288 6.075-5.567-.083-11.13-.58-12.362-1.106zm149.22-99.209c0-3.474-1.08-4.9-4.042-5.346-2.424-.364-4.473.585-5.12 2.37-1.785 4.918.716 8.983 5.12 8.322 2.963-.445 4.041-1.872 4.041-5.346zM20.022 98.047c-4.74-2.634-7.103-11.762-3.918-15.13.957-1.012 1.317-5.17.801-9.239C14.093 51.5 30.352 30.775 56.294 23.472c19.91-5.606 34.344-2.15 48.18 11.537 4.73 4.68 9.344 8.508 10.253 8.508.908 0 2.629 2.495 3.823 5.545 3.083 7.87-.388 14.462-12.208 23.191-23.908 17.654-74.022 32.63-86.32 25.794z",
};

pub const FADE_OUT: Icon = Icon {
    width: 135,
    height: 135,
    path: "M130.43 120.33h.8c.38 0 .7.31.7.7v13.6a.7.7 0 01-.7.7h-.8a.7.7 0 01-.7-.7v-13.6c0-.39.31-.7.7-.7zm0-24.02h.8c.38 0 .7.31.7.7v13.6a.7.7 0 01-.7.7h-.8a.7.7 0 01-.7-.7V97c0-.39.31-.7.7-.7zm0-24.02h.8c.38 0 .7.31.7.7v13.6a.7.7 0 01-.7.7h-.8a.7.7 0 01-.7-.7v-13.6c0-.39.31-.7.7-.7zm-.07-24.02h.8c.39 0 .7.32.7.7v13.6a.7.7 0 01-.7.7h-.8a.7.7 0 01-.7-.7v-13.6c0-.38.32-.7.7-.7zm0-24.01h.8c.39 0 .7.3.7.7v13.6a.7.7 0 01-.7.69h-.8a.7.7 0 01-.7-.7v-13.6c0-.38.32-.7.7-.7zm0-24.02h.8c.39 0 .7.3.7.7v13.6a.7.7 0 01-.7.7h-.8a.7.7 0 01-.7-.7V.93c0-.4.32-.7.7-.7zm-18.9 112.23h1.8c.88 0 1.58.48 1.58 1.07v20.71c0 .6-.7 1.07-1.58 1.07h-1.8c-.88 0-1.58-.48-1.58-1.07v-20.71c0-.6.7-1.07 1.58-1.07zm.4-37.4h1.8c.87 0 1.58.47 1.58 1.06v20.71c0 .6-.7 1.07-1.58 1.07h-1.8c-.88 0-1.59-.48-1.59-1.07V76.13c0-.6.7-1.07 1.58-1.07zm0-37.42h1.8c.87 0 1.58.48 1.58 1.07v20.71c0 .6-.7 1.07-1.58 1.07h-1.8c-.88 0-1.59-.48-1.59-1.07V38.72c0-.6.7-1.07 1.58-1.07zm-.2-37.4h1.8c.88 0 1.58.47 1.58 1.06v20.71c0 .6-.7 1.07-1.58 1.07h-1.8c-.88 0-1.58-.48-1.58-1.07V1.31c0-.6.7-1.07 1.58-1.07zM90.67 101.3h3.05c1.48 0 2.67.7 2.67 1.59v30.84c0 .88-1.19 1.58-2.67 1.58h-3.05c-1.48 0-2.67-.7-2.67-1.58v-30.84c0-.88 1.2-1.59 2.67-1.59zm0-50.53h3.05c1.48 0 2.67.71 2.67 1.59V83.2c0 .88-1.19 1.59-2.67 1.59h-3.05c-1.48 0-2.67-.71-2.67-1.59V52.36c0-.88 1.2-1.59 2.67-1.59zm0-50.52h3.05c1.48 0 2.67.7 2.67 1.58v30.85c0 .87-1.19 1.58-2.67 1.58h-3.05c-1.48 0-2.67-.7-2.67-1.58V1.83C88 .95 89.2.25 90.67.25zM60.1.3h5.72c3.48 0 6.29 2.8 6.29 6.29v122.38c0 3.49-2.8 6.3-6.3 6.3H60.1a6.27 6.27 0 01-6.3-6.3V6.6C53.8 3.1 56.6.3 60.1.3zM4.9.36h36.57c2.5 0 4.51 2.8 4.51 6.28v122.28c0 3.48-2.01 6.28-4.51 6.28H4.9c-2.5 0-4.5-2.8-4.5-6.28V6.64C.4 3.16 2.4.36 4.9.36z",
};

pub const BIG_CIRCLE: Icon = Icon {
    width: 135,
    height: 135,
    path: "M122.1 67.73a54.36 54.36 0 01-54.37 54.37 54.36 54.36 0 01-54.36-54.37 54.36 54.36 0 0154.36-54.36 54.36 54.36 0 0154.37 54.36z",
};

pub const MEDIUM_CIRCLE: Icon = Icon {
    width: 135,
    height: 135,
    path: "M100.57 67.73a32.84 32.84 0 01-32.84 32.84A32.84 32.84 0 0134.9 67.73 32.84 32.84 0 0167.73 34.9a32.84 32.84 0 0132.84 32.83z",
};

pub const SMALL_CIRCLE: Icon = Icon {
    width: 135,
    height: 135,
    path: "M75.07 67.73a7.34 7.34 0 01-7.34 7.34 7.34 7.34 0 01-7.33-7.34 7.34 7.34 0 017.33-7.33 7.34 7.34 0 017.34 7.33z",
};