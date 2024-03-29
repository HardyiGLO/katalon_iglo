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

var1 = WS.sendRequestAndVerify(findTestObject(null))

def xmlbody = var1.responseBodyContent

def value = new XmlSlurper().parseText(xmlbody)

//Cetak hasil uraian data dr api///
System.out.printIn(value)

//Put hasil dr uraian data dr variabel varValue ke GlobalVariable//
GlobalVariable.FIRST_NUM = value

//Cetak nilai dr global variable
System.out.printIn(GlobalVariable.FIRST_NUM)

WS.sendRequestAndVerify(findTestObject('Katalon Enterprise Feature/SOAP Service/SOAPDemoSoap/DivideInteger', [('vnum') : GlobalVariable.FIRST_NUM]))

