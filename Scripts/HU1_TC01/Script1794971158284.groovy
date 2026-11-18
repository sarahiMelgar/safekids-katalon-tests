import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.llm.keyword.LlmKeywords as LLM
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

<<<<<<< HEAD:Scripts/h1_TC01/Script1794971158284.groovy
WebUI.openBrowser(null)

WebUI.navigateToUrl('https://safekids-17k2.onrender.com')

WebUI.setText(findTestObject('Page_Login/Page_Login/input_correoejemplo.com'), 'padre@safekids.com')

WebUI.setEncryptedText(findTestObject('Page_Login/Page_Login/input_Tu contrasea'), '9OZSCXPBzZuqXCLJW9DsVg==')

WebUI.click(findTestObject('Page_Login/Page_Login/div_Iniciar sesin'))

WebUI.setText(findTestObject('Page_Login/Page_Login/input_correoejemplo.com'), 'padre@safekids.com')

WebUI.setEncryptedText(findTestObject('Page_Login/Page_Login/input_Tu contrasea'), '9OZSCXPBzZuqXCLJW9DsVg==')

WebUI.click(findTestObject('Page_Login/Page_Login/div_Iniciar sesin_1'))
=======
WebUI.openBrowser('https://safekids-17k2.onrender.com/')
>>>>>>> 8b21419226d89bc86bfc865e923ebf12c8153fc7:Scripts/HU1_TC01/Script1794971158284.groovy

