import os
import xml.etree.ElementTree as ET
from jinja2 import Environment,FileSystemLoader
from google.cloud import storage;

env = Environment(loader=FileSystemLoader('./'))
template = env.get_template("template/report_template.html")
def generate_html_report(xml_floder,output_file="contract-test-report/index.html"):
    
    passed_tests =0
    failed_tests=0
    time_total=0

    test_results = []

    for filename in os.listdir(xml_floder):
        if filename.index(".xml") !=-1:
          filePath = os.path.join(xml_floder,filename)
          tree = ET.parse(filePath)
          root = tree.getroot()

          attrib_dict=root.attrib
          failures = int(attrib_dict.get('failures'))
          passed_tests += int (attrib_dict.get('tests'))-failures
          failed_tests += failures

          test_name = attrib_dict.get('name')
          tests = int(attrib_dict.get('tests'))
          errors = int(attrib_dict.get('errors'))
          skipped = int(attrib_dict.get('skipped'))
          time = float(attrib_dict.get('time'))
          time_total+=time

          test_cases =[]
          deatiled_message=""
          symbols="" 
          for testcase in root.iter('testcase'):
            symbols="✅" 
            deatiled_message=""
            classname = testcase.get('classname')
            name = testcase.get('name')
            result = 'Passed'
            if testcase.find('failure') is not None:
                result = 'Failed'
                deatiled_message = testcase.find('failure').text.strip()
                symbols="❌"
            elif testcase.find('error') is not None:
                result ='Error'
                deatiled_message = testcase.find('failure').text.strip()
                symbols="❗⚠️"
            elif testcase.find('skipped') is not None:
                result = 'Skipped'
                deatiled_message = testcase.find('failure').text.strip()
                symbols="⏩"
            test_cases.append({

            'classname':classname,
            'name':symbols+"      "+name,
            'result':result,
            'failure_message': testcase.find('failure').get('message') if testcase.find('failure')is not None else '',
            
            'error_message': testcase.find('error').get('message') if testcase.find('error')is not None else '',

            'skipped_message': testcase.find('skipped').get('message') if testcase.find('skipped')is not None else '',
            'time': float(testcase.get('time')),
            'deatiled_message':deatiled_message
            
            })
            
          test_results.append({
            'name':test_name,
            'tests': tests,
            'failures':failed_tests,
            'errors':errors,
            'skipped':skipped,
            'time':time,
            'test_case': test_cases
            })
    html_report = template.render(test_results=test_results,passed_tests=passed_tests,failed_tests=failed_tests,time_total=time_total)
    with open(output_file,'w') as f:
        f.write(html_report)
    
    print(f"HTML report generated successfully :{output_file}")

def get_len(obj):
    return len(obj)>0

# def download_floder_from_gcs(bucket_name,source_folder,destination_floder):
#     storage_client = storage.Client()
#     bucket = storage_client.bucket(bucket_name)
#     blobs = bucket.list_blobs(prefix=)
env.globals['object_len']=get_len
generate_html_report("reports")