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

WebUI.openBrowser(null)

WebUI.navigateToUrl('https://safekids-17k2.onrender.com/')

WebUI.click(findTestObject('Page_Login/div_Registrarme'))

WebUI.click(findTestObject('Page_Registro/div_'))

WebUI.setText(findTestObject('Page_Registro/input_Tu nombre'), 'Estefani Mercado Mendez')

WebUI.setText(findTestObject('Page_Registro/input_correoejemplo.com'), 'mercadofanny02@gmail.com')

WebUI.setText(findTestObject('Page_Registro/input_10 dgitos'), '551234567')

WebUI.setEncryptedText(findTestObject('Page_Registro/input_8 caracteres, mayscula y smbolo'), '7/qV00sTttM+8jf1Eleogw==')

WebUI.click(findTestObject('Page_Registro/div__1'))

WebUI.click(findTestObject('Page_Registro/div_css-g5y9jx r-1awozwy r-14lw9ot r-12uivnu r-k'))

WebUI.click(findTestObject('Page_Registro/div_Registrarme'))

