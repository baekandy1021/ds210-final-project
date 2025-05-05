import pandas as pd
import os
from datetime import datetime
#improting csv
df = pd.read_csv("soc-sign-bitcoinotc.csv", header=None, names=["source", "target", "rating", "time"])

#data cleaning
df = df.dropna()
df['source'] = df['source'].astype(int)
df['target'] = df['target'].astype(int)
df['time'] = df['time'].astype(float).astype(int)
df['year'] = df['time'].apply(lambda t: datetime.utcfromtimestamp(t).year)
os.makedirs("bitcoinotc_by_year", exist_ok=True)

#writing csv 
for year, group in df.groupby('year'):
    filename = f"bitcoinotc_by_year/bitcoinotc_{year}.csv"
    group.drop(columns=['year']).to_csv(filename, index=False, header=False)
    print(f"Saved {filename} ({len(group)} rows)")