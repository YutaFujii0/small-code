from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.common.by import By
import time

TARGET_DATE = '20231227'
URL_TOP = f'https://reserve.tokyodisneyresort.jp'
URL_SEARCH = f'https://reserve.tokyodisneyresort.jp/hotel/list/?showWay=&roomsNum=1&adultNum=2&childNum=0&stayingDays=1&useDate={TARGET_DATE}&cpListStr=&childAgeBedInform=&searchHotelCD=DHM&searchHotelDiv=&hotelName=&searchHotelName=&searchLayer=&searchRoomName=&hotelSearchDetail=true&checkPointStr=&displayType=data-hotel&reservationStatus=1'
URL_WAITING_QUEUE = 'https://reserve-q.tokyodisneyresort.jp/'

SELECTOR_HOTEL = '#header > nav > ul > li:nth-child(2) > a > img'
SELECTOR_HOTEL_SEARCH = '#js-callHotelSearch > a'
SELECTOR_HOTEL_SEARCH_MIRACOSTA = '#js-hotelHotelSearch > div.boxModal19 > div > div > ul > li:nth-child(4) > a > span:nth-child(2)'

NUMBER_OF_TABS = 1
INTERVAL = 60  # seconds

ELEMENT_1 = '#section_スペチアーレ・ルーム＆スイート　ポルト・パラディーゾ・サイド\ バルコニールーム\ ハーバービュー > div > div.new-ui-theme > ul > li:nth-child(2) > a'



# Path to your ChromeDriver
chrome_driver_path = './chromedriver'
user_data_dir = '/Users/yuta/Library/Application Support/Google/Chrome'
profile_dir = 'Profile 1'

options = Options()
options.add_argument(f'user-data-dir={user_data_dir}')
options.add_argument(f'profile-directory={profile_dir}')
options.add_experimental_option("excludeSwitches", ["enable-automation"])
options.add_experimental_option('useAutomationExtension', False)

# Set up the Chrome WebDriver with Service
service = Service(executable_path=chrome_driver_path)
driver = webdriver.Chrome(service=service, options=options)

time.sleep(10)

# clear the cookies of the specific domain A
driver.delete_all_cookies()

# Open the first tab with the top page
driver.get(URL_TOP)

# click 'hote' tab
driver.find_element(By.CSS_SELECTOR, SELECTOR_HOTEL).click()
time.sleep(10)
driver.find_element(By.CSS_SELECTOR, SELECTOR_HOTEL_SEARCH).click()
time.sleep(10)
driver.find_element(By.CSS_SELECTOR, SELECTOR_HOTEL_SEARCH_MIRACOSTA).click()


# Open remaining NUMBER_OF_TABS tabs
# for _ in range(NUMBER_OF_TABS - 1):
#     driver.execute_script("window.open('');")  # Open a new tab
#     driver.switch_to.window(driver.window_handles[-1])  # Switch to the new tab
#     driver.get(URL)
#     time.sleep(1)

# Function to refresh a specific tab
def refresh_tab(tab_index):
    driver.switch_to.window(driver.window_handles[tab_index])
    driver.refresh()

def query_availabilities():
    availabilities = []
    for i in range(NUMBER_OF_TABS):
        driver.switch_to.window(driver.window_handles[i])
        if URL_WAITING_QUEUE in driver.current_url:
            availabilities.append(False)
        else:
            availabilities.append(True)
    return availabilities

# Refreshing each tab every second in a round-robin fashion
while True:
    time.sleep(INTERVAL)
    for i in range(NUMBER_OF_TABS):
        # if tab's current URL contains URL_WAITING_QUEUE then skip
        if URL_WAITING_QUEUE in driver.current_url:
            continue
        else:
            # check ELEMENT_1 is visible, if yes, click it
            # if driver.find_element_by_css_selector(ELEMENT_1).is_displayed():
            #     driver.find_element_by_css_selector(ELEMENT_1).click()
            refresh_tab(i)
            time.sleep(1)  # Wait for 1 second before refreshing the next tab
