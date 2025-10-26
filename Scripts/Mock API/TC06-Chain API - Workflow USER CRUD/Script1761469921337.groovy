import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testobject.ResponseObject

def testFirtsName = 'Fitri'
def testLastName = 'Cynthia'
def responsePost= WS.sendRequest(findTestObject('POST NEW USER', [('firstname') : testFirtsName, ('lastname') : testLastName, ('username') : 'broot', ('jobposition') : 'QAE'
            , ('joblevel') : 'Junior', ('salary') : 7777777, ('workduration') : 2]))
def userName = WS.getElementPropertyValue(responsePost, '[0].username')
def jobPosition = WS.getElementPropertyValue(responsePost, '[0].job_position')

WS.verifyEqual(userName, 'broot')
assert jobPosition == 'QAE'

def responseAfterPost= WS.sendRequest(findTestObject('GET SINGLE USER'))

def firtsNameAfterPost = WS.getElementPropertyValue(responseAfterPost, '[0].first_name')
def lastNameAfterPost = WS.getElementPropertyValue(responseAfterPost, '[0].last_name')
def workDurationAfterPost = WS.getElementPropertyValue(responseAfterPost, '[0].work_duration')

WS.verifyEqual(firtsNameAfterPost, testFirtsName)
WS.verifyEqual(lastNameAfterPost, testLastName)
assert workDurationAfterPost == 2

def responsePatch= WS.sendRequest(findTestObject('PATCH A USER', [('jobposition') : 'Senior QA Engineer', ('salary') : 8888888]))

def userNamePatch = WS.getElementPropertyValue(responsePatch, '[0].username')
def jobPositionPatch = WS.getElementPropertyValue(responsePatch, '[0].job_position')
def SalaryPatch = WS.getElementPropertyValue(responsePatch, '[0].salary')

WS.verifyEqual(userNamePatch, 'broot')
WS.verifyMatch(jobPositionPatch, 'Senior QA Engineer', false)
assert SalaryPatch == 8888888

def responseAfterPatch = WS.sendRequest(findTestObject('GET SINGLE USER'))

def userNameAfterPatch = WS.getElementPropertyValue(responseAfterPatch, '[0].username')
def firtsNameAfterPatch = WS.getElementPropertyValue(responseAfterPatch, '[0].first_name')
def lastNameAfterPatch = WS.getElementPropertyValue(responseAfterPatch, '[0].last_name')
def jobPositionAfterPatch = WS.getElementPropertyValue(responseAfterPatch, '[0].job_position')
def SalaryAfterPatch = WS.getElementPropertyValue(responseAfterPatch, '[0].salary')

WS.verifyEqual(userNameAfterPatch, 'broot')
WS.verifyEqual(firtsNameAfterPatch, testFirtsName)
WS.verifyEqual(lastNameAfterPatch, testLastName)
WS.verifyMatch(jobPositionAfterPatch, 'Senior QA Engineer', false)
assert SalaryAfterPatch == 8888888

//WS.sendRequest(findTestObject('DELETE USER'))

ResponseObject response = WS.sendRequest(findTestObject('DELETE USER'))

def responseTimeDelete = response.getElapsedTime()
WS.verifyLessThan(responseTimeDelete, 10000)


