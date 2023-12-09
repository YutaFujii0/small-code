from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.chrome.options import Options
import time

TARGET_DATE = '20231226'
URL = f'https://reserve.tokyodisneyresort.jp/hotel/list/?showWay=&roomsNum=1&adultNum=2&childNum=0&stayingDays=1&useDate={TARGET_DATE}&cpListStr=&childAgeBedInform=&searchHotelCD=DHM&searchHotelDiv=&hotelName=&searchHotelName=&searchLayer=&searchRoomName=&hotelSearchDetail=true&checkPointStr=&displayType=data-hotel&reservationStatus=1'
NUMBER_OF_TABS = 30

# Path to your ChromeDriver
chrome_driver_path = './chromedriver'
profile_path = '/Users/yuta/Library/Application\ Support/Google/Chrome/Profile\ 1'

options = Options()
options.add_argument(f'user-data-dir={profile_path}')

# Set up the Chrome WebDriver with Service
service = Service(executable_path=chrome_driver_path)
driver = webdriver.Chrome(service=service, options=options)

# Open the first tab with the URL
driver.get(URL)
time.sleep(1)  # Waiting 1 second before opening the next tab

# Open remaining 29 tabs
for _ in range(NUMBER_OF_TABS - 1):
    driver.execute_script("window.open('');")  # Open a new tab
    driver.switch_to.window(driver.window_handles[-1])  # Switch to the new tab
    driver.get('your_url')
    time.sleep(1)

# Function to refresh a specific tab
def refresh_tab(tab_index):
    driver.switch_to.window(driver.window_handles[tab_index])
    driver.refresh()

# Refreshing each tab every second in a round-robin fashion
while True:
    for i in range(NUMBER_OF_TABS):  # Assuming 30 tabs
        refresh_tab(i)
        time.sleep(1)  # Wait for 1 second before refreshing the next tab
