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

WebUI.click(findTestObject('Page_Login/div_'))

WebUI.setText(findTestObject('Page_Login/input_correoejemplo.com'), 'lh425981@gmail.com')

WebUI.click(findTestObject('Page_Login/input_Tu contrasea'))

WebUI.setText(findTestObject('Page_Login/input_correoejemplo.com'), 'lh425081@gmail.com')

WebUI.click(findTestObject('Page_Login/div__1'))

WebUI.setEncryptedText(findTestObject('Page_Login/input_Tu contrasea'), 'bzJtPPkZlzcpV+ayg+0pGg==')

WebUI.click(findTestObject('Page_Login/div_Iniciar sesin'))

WebUI.click(findTestObject('Page_Principal/div_Historial'))

WebUI.click(findTestObject('Page_Principal/div_'))

WebUI.doubleClick(findTestObject('Page_Principal/div_'))

WebUI.click(findTestObject('Page_Principal/div_'))

WebUI.doubleClick(findTestObject('Page_Principal/div_'))

WebUI.click(findTestObject('Page_Principal/div_'))

WebUI.doubleClick(findTestObject('Page_Principal/div_'))

WebUI.click(findTestObject('Page_Principal/div_'))

WebUI.click(findTestObject('Page_Principal/div_Martes, 17 de noviembre de 2026'))

WebUI.doubleClick(findTestObject('Page_Principal/div_Martes, 17 de noviembre de 2026'))

WebUI.click(findTestObject('Page_Principal/div__1'))

WebUI.click(findTestObject('Page_Principal/div__2'))

WebUI.click(findTestObject('Page_Principal/div_Martes, 17 de noviembre de 2026_1'))

WebUI.doubleClick(findTestObject('Page_Principal/div_Martes, 17 de noviembre de 2026_1'))

WebUI.click(findTestObject('Page_Principal/div_'))

WebUI.doubleClick(findTestObject('Page_Principal/div_'))

WebUI.click(findTestObject('Page_Principal/div_'))

WebUI.click(findTestObject('Page_Principal/div_Martes, 17 de noviembre de 2026'))

WebUI.click(findTestObject('Page_Principal/div_'))

WebUI.doubleClick(findTestObject('Page_Principal/div_'))

WebUI.click(findTestObject('Page_Principal/div_'))

WebUI.closeBrowser()

