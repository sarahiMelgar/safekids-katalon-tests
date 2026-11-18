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

WebUI.navigateToUrl('https://safekids-17k2.onrender.com/inicio')

WebUI.setText(findTestObject('Page_Login/input_correoejemplo.com'), 'padre@safekids.com')

WebUI.setEncryptedText(findTestObject('Page_Login/input_Tu contrasea'), '9OZSCXPBzZuqXCLJW9DsVg==')

WebUI.click(findTestObject('Page_Login/div_Iniciar sesin'))

WebUI.click(findTestObject('Page_Principal/div_Prediccin'))

WebUI.click(findTestObject('Page_Principal/div_CON SEAL'))

WebUI.click(findTestObject('Page_Principal/div_ltima ubicacin conocida_ Casa'))

WebUI.click(findTestObject('Page_Principal/div_Zona alcanzable si se perdiera la seal_ rad'))

WebUI.click(findTestObject('Page_Principal/div_Prediccin poco precisa_ hay pocas visitas'))

WebUI.click(findTestObject('Page_Principal/path_leaflet-interactive'))

WebUI.click(findTestObject('Page_Principal/div_div'))

