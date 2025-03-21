|                           |         |                             |      |            |           |             |               |
|---------------------------|---------|-----------------------------|------|------------|-----------|-------------|---------------|
| I2C_MAP                   |         |                             |      |            |           |             |               |
|                           |         |                             |      |            |           |             |               |
| Address width             | 8       |                             |      |            |           |             |               |
| Data width                | 16      |                             |      |            |           |             |               |
|                           |         |                             |      |            |           |             |               |
| Register Name             | Address | Field Name                  | Type | Bit Offset | Bit Width | Reset Value | Reset Sources |
| CHIPID                    | 0x0     |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B11                  | RO   | 11         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B10                  | RO   | 10         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B9                   | RO   | 9          | 1         | 0x0         | POR           |
|                           |         | UNUSED_B8                   | RO   | 8          | 1         | 0x0         | POR           |
|                           |         | SILICON_ID                  | RO   | 4          | 4         | X           | X             |
|                           |         | METAL_ID                    | RO   | 0          | 4         | X           | X             |
| CFG3_ACT                  | 0x01    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B11                  | RO   | 11         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B10                  | RO   | 10         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B9                   | RO   | 9          | 1         | 0x0         | POR           |
|                           |         | UNUSED_B8                   | RO   | 8          | 1         | 0x0         | POR           |
|                           |         | UNUSED_B7                   | RO   | 7          | 1         | 0x1         | POR           |
|                           |         | CHG_ON                      | RW   | 6          | 1         | 0x0         | POR           |
|                           |         | DCHG_ON                     | RW   | 5          | 1         | 0x0         | POR           |
|                           |         | BAL5_ON                     | RW   | 4          | 1         | 0x0         | POR           |
|                           |         | BAL4_ON                     | RW   | 3          | 1         | 0x0         | POR           |
|                           |         | BAL3_ON                     | RW   | 2          | 1         | 0x0         | POR           |
|                           |         | BAL2_ON                     | RW   | 1          | 1         | 0x0         | POR           |
|                           |         | BAL1_ON                     | RW   | 0          | 1         | 0x0         | POR           |
| CFG1_FILTERS_CYCLES       | 0x02    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | T_MEAS_CYCLE                | RW   | 7          | 5         | 0x0         | POR           |
|                           |         | T_CUR_FILTER                | RW   | 5          | 2         | 0x0         | POR           |
|                           |         | T_SC_FILTER                 | RW   | 2          | 3         | 0x0         | POR           |
|                           |         | TCELL_FILTER                | RW   | 0          | 2         | 0x0         | POR           |
| DEV_ADDR                  | 0x03    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B11                  | RO   | 11         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B10                  | RO   | 10         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B9                   | RO   | 9          | 1         | 0x0         | POR           |
|                           |         | UNUSED_B8                   | RO   | 8          | 1         | 0x0         | POR           |
|                           |         | UNUSED_B7                   | RO   | 7          | 1         | 0x0         | POR           |
|                           |         | DEV_ADDR_ID                 | RW   | 0          | 7         | 0x49        | POR           |
| CFG2_ENABLES              | 0x04    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | CRC_EN                      | RW   | 13         | 1         | 0x0         | POR           |
|                           |         | CHG_HS_LS                   | RW   | 12         | 1         | 0x1         | POR           |
|                           |         | DCHG_HS_LS                  | RW   | 11         | 1         | 0x1         | POR           |
|                           |         | SC_EN                       | RW   | 10         | 1         | 0x0         | POR           |
|                           |         | OVC_EN                      | RW   | 9          | 1         | 0x0         | POR           |
|                           |         | CC_ACC_EN                   | RW   | 8          | 1         | 0x0         | POR           |
|                           |         | CSA_EN                      | RW   | 7          | 1         | 0x0         | POR           |
|                           |         | NTC_EN                      | RW   | 6          | 1         | 0x0         | POR           |
|                           |         | VB_EN                       | RW   | 5          | 1         | 0x0         | POR           |
|                           |         | VCELL_EN_5                  | RW   | 4          | 1         | 0x0         | POR           |
|                           |         | VCELL_EN_4                  | RW   | 3          | 1         | 0x0         | POR           |
|                           |         | VCELL_EN_3                  | RW   | 2          | 1         | 0x0         | POR           |
|                           |         | VCELL_EN_2                  | RW   | 1          | 1         | 0x0         | POR           |
|                           |         | VCELL_EN_1                  | RW   | 0          | 1         | 0x0         | POR           |
| CSA_GAIN_FACTOR           | 0x05    |                             |      |            |           |             |               |
|                           |         | CSA_GAIN_FACTOR             | RW   | 0          | 16        | 0x8000      | POR           |
| VCELL_OV_TH               | 0x06    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | NCELL_OV_CNT_TH             | RW   | 8          | 4         | 0x0         | POR           |
|                           |         | VCELL_OV_TH                 | RW   | 0          | 8         | 0x0         | POR           |
| VCELL_UV_TH               | 0x07    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | NCELL_UV_CNT_TH             | RW   | 8          | 4         | 0x0         | POR           |
|                           |         | VCELL_UV_TH                 | RW   | 0          | 8         | 0x0         | POR           |
| VCELL_SEVERE_DELTA_THRS   | 0x08    |                             |      |            |           |             |               |
|                           |         | VCELL_SEVERE_UV_DELTA_TH    | RW   | 8          | 8         | 0x0         | POR           |
|                           |         | VCELL_SEVERE_OV_DELTA_TH    | RW   | 0          | 8         | 0x0         | POR           |
| VCELL_BAL_UV_DELTA_TH     | 0x09    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | NCELL_BAL_UV_CNT_TH         | RW   | 8          | 4         | 0x0         | POR           |
|                           |         | VCELL_BAL_UV_DELTA_TH       | RW   | 0          | 8         | 0x0         | POR           |
| VB_OV_TH                  | 0x0A    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | NVB_OV_CNT_TH               | RW   | 8          | 4         | 0x0         | POR           |
|                           |         | VB_OV_TH                    | RW   | 0          | 8         | 0x0         | POR           |
| VB_UV_TH                  | 0x0B    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | NVB_UV_CNT_TH               | RW   | 8          | 4         | 0x0         | POR           |
|                           |         | VB_UV_TH                    | RW   | 0          | 8         | 0x0         | POR           |
| VB_SUM_MAX_DIFF_TH        | 0x0C    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B11                  | RO   | 11         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B10                  | RO   | 10         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B9                   | RO   | 9          | 1         | 0x0         | POR           |
|                           |         | UNUSED_B8                   | RO   | 8          | 1         | 0x0         | POR           |
|                           |         | VB_SUM_MAX_DIFF_TH          | RW   | 0          | 8         | 0x0         | POR           |
| VNTC_OT_TH                | 0x0D    |                             |      |            |           |             |               |
|                           |         | NNTC_OT_CNT_TH              | RW   | 12         | 4         | 0x0         | POR           |
|                           |         | NTC_OT_TH                   | RW   | 0          | 12        | 0x0         | POR           |
| VNTC_UT_TH                | 0x0E    |                             |      |            |           |             |               |
|                           |         | NNTC_UT_CNT_TH              | RW   | 12         | 4         | 0x0         | POR           |
|                           |         | NTC_UT_TH                   | RW   | 0          | 12        | 0x0         | POR           |
| VNTC_SEVERE_OT_DELTA_TH   | 0x0F    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | NTC_SEVERE_OT_DELTA_TH      | RW   | 0          | 12        | 0x0         | POR           |
| OVC_THRESHOLDS            | 0x10    |                             |      |            |           |             |               |
|                           |         | OVC_DCHG_TH                 | RW   | 8          | 8         | 0x0         | POR           |
|                           |         | OVC_CHG_TH                  | RW   | 0          | 8         | 0x0         | POR           |
| PERSISTENT_OVC_THRESHOLDS | 0x11    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B11                  | RO   | 11         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B10                  | RO   | 10         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B9                   | RO   | 9          | 1         | 0x0         | POR           |
|                           |         | UNUSED_B8                   | RO   | 8          | 1         | 0x0         | POR           |
|                           |         | PERSIST_OVC_TH              | RW   | 0          | 8         | 0x0         | POR           |
| SC_THRESHOLD              | 0x12    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B11                  | RO   | 11         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B10                  | RO   | 10         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B9                   | RO   | 9          | 1         | 0x0         | POR           |
|                           |         | UNUSED_B8                   | RO   | 8          | 1         | 0x0         | POR           |
|                           |         | SC_PERSIST_TH               | RW   | 4          | 4         | 0x0         | POR           |
|                           |         | SC_TH                       | RW   | 0          | 4         | 0x0         | POR           |
| TO_PRDRV_BAL_MSK          | 0x13    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | VB_SUM_CHECK_BAL_MSK        | RW   | 14         | 1         | 0x0         | POR           |
|                           |         | DIE_OT_BAL_MSK              | RW   | 13         | 1         | 0x0         | POR           |
|                           |         | NTC_SEVERE_OT_BAL_MSK       | RW   | 12         | 1         | 0x0         | POR           |
|                           |         | BAL_UV_BAL_MSK              | RW   | 11         | 1         | 0x0         | POR           |
|                           |         | DIE_OT_PRDRV_MSK            | RW   | 10         | 1         | 0x0         | POR           |
|                           |         | NTC_UT_PRDRV_MSK            | RW   | 9          | 1         | 0x0         | POR           |
|                           |         | NTC_SEVERE_OT_PRDRV_MSK     | RW   | 8          | 1         | 0x0         | POR           |
|                           |         | NTC_OT_PRDRV_MSK            | RW   | 7          | 1         | 0x0         | POR           |
|                           |         | VB_SUM_CHECK_PRDRV_MSK      | RW   | 6          | 1         | 0x0         | POR           |
|                           |         | VB_OV_PRDRV_MSK             | RW   | 5          | 1         | 0x0         | POR           |
|                           |         | VB_UV_PRDRV_MSK             | RW   | 4          | 1         | 0x0         | POR           |
|                           |         | CELL_SEVERE_OV_PRDRV_MSK    | RW   | 3          | 1         | 0x0         | POR           |
|                           |         | CELL_OV_PRDRV_MSK           | RW   | 2          | 1         | 0x0         | POR           |
|                           |         | CELL_SEVERE_UV_PRDRV_MSK    | RW   | 1          | 1         | 0x0         | POR           |
|                           |         | CELL_UV_PRDRV_MSK           | RW   | 0          | 1         | 0x0         | POR           |
| TO_FUSE_RST_MSK           | 0x14    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B11                  | RO   | 11         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B10                  | RO   | 10         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B9                   | RO   | 9          | 1         | 0x0         | POR           |
|                           |         | UNUSED_B8                   | RO   | 8          | 1         | 0x0         | POR           |
|                           |         | UNUSED_B7                   | RO   | 7          | 1         | 0x0         | POR           |
|                           |         | VB_OV_RST_MSK               | RW   | 6          | 1         | 0x0         | POR           |
|                           |         | CELL_SEVERE_OV_RST_MSK      | RW   | 5          | 1         | 0x0         | POR           |
|                           |         | CELL_OV_RST_MSK             | RW   | 4          | 1         | 0x0         | POR           |
|                           |         | NTC_SEVERE_OT_FUSE_MSK      | RW   | 3          | 1         | 0x0         | POR           |
|                           |         | VB_SUM_CHECK_FUSE_MSK       | RW   | 2          | 1         | 0x0         | POR           |
|                           |         | CELL_SEVERE_OV_FUSE_MSK     | RW   | 1          | 1         | 0x0         | POR           |
|                           |         | CELL_SEVERE_UV_FUSE_MSK     | RW   | 0          | 1         | 0x0         | POR           |
| TO_FAULTN_MSK             | 0x15    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | DIE_OT_FAULTN_MSK           | RW   | 11         | 1         | 0x0         | POR           |
|                           |         | NTC_UT_FAULTN_MSK           | RW   | 10         | 1         | 0x0         | POR           |
|                           |         | NTC_SEVERE_OT_FAULTN_MSK    | RW   | 9          | 1         | 0x0         | POR           |
|                           |         | NTC_OT_FAULTN_MSK           | RW   | 8          | 1         | 0x0         | POR           |
|                           |         | VB_SUM_CHECK_FAULTN_MSK     | RW   | 7          | 1         | 0x0         | POR           |
|                           |         | VB_OV_FAULTN_MSK            | RW   | 6          | 1         | 0x0         | POR           |
|                           |         | VB_UV_FAULTN_MSK            | RW   | 5          | 1         | 0x0         | POR           |
|                           |         | BAL_UV_FAULTN_MSK           | RW   | 4          | 1         | 0x0         | POR           |
|                           |         | CELL_SEVERE_OV_FAULTN_MSK   | RW   | 3          | 1         | 0x0         | POR           |
|                           |         | CELL_OV_FAULTN_MSK          | RW   | 2          | 1         | 0x0         | POR           |
|                           |         | CELL_SEVERE_UV_FAULTN_MSK   | RW   | 1          | 1         | 0x0         | POR           |
|                           |         | CELL_UV_FAULTN_MSK          | RW   | 0          | 1         | 0x0         | POR           |
| CURR_MSK                  | 0x16    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | PERSIST_OVC_DCHG_FAULTN_MSK | RW   | 11         | 1         | 0x0         | POR           |
|                           |         | PERSIST_OVC_CHG_FAULTN_MSK  | RW   | 10         | 1         | 0x0         | POR           |
|                           |         | PERSIST_SC_DCHG_FAULTN_MSK  | RW   | 9          | 1         | 0x0         | POR           |
|                           |         | SC_DCHG_FAULTN_MSK          | RW   | 8          | 1         | 0x0         | POR           |
|                           |         | OVC_DCHG_FAULTN_MSK         | RW   | 7          | 1         | 0x0         | POR           |
|                           |         | OVC_CHG_FAULTN_MSK          | RW   | 6          | 1         | 0x0         | POR           |
|                           |         | PERSIST_SC_DCHG_FUSE_MSK    | RW   | 5          | 1         | 0x0         | POR           |
|                           |         | PERSIST_OVC_DCHG_FUSE_MSK   | RW   | 4          | 1         | 0x0         | POR           |
|                           |         | PERSIST_OVC_CHG_FUSE_MSK    | RW   | 3          | 1         | 0x0         | POR           |
|                           |         | SC_DCHG_PRDRV_MSK           | RW   | 2          | 1         | 0x0         | POR           |
|                           |         | OVC_DCHG_PRDRV_MSK          | RW   | 1          | 1         | 0x0         | POR           |
|                           |         | OVC_CHG_PRDRV_MSK           | RW   | 0          | 1         | 0x0         | POR           |
| MANUFACTURER_NAME_MSB     | 0x17    |                             |      |            |           |             |               |
|                           |         | MANUFACTURER_NAME_LSB       | RW   | 0          | 16        | 0x0         | POR           |
| MANUFACTURER_NAME_LSB     | 0x18    |                             |      |            |           |             |               |
|                           |         | MANUFACTURER_NAME_MSB       | RW   | 0          | 16        | 0x0         | POR           |
| MANUFACTURING_DATE        | 0x19    |                             |      |            |           |             |               |
|                           |         | MANUFACTURING_DATE          | RW   | 0          | 16        | 0x0         | POR           |
| FIRST_USAGE_DATE          | 0x1A    |                             |      |            |           |             |               |
|                           |         | FIRST_USAGE_DATE            | RW   | 0          | 16        | 0x0         | POR           |
| SERIAL_NUMBER_MSB         | 0x1B    |                             |      |            |           |             |               |
|                           |         | SERIAL_NUMBER_MSB           | RW   | 0          | 16        | 0x0         | POR           |
| SERIAL_NUMBER_LSB         | 0x1C    |                             |      |            |           |             |               |
|                           |         | SERIAL_NUMBER_LSB           | RW   | 0          | 16        | 0x0         | POR           |
| DEVICE_NAME_MSB           | 0x1D    |                             |      |            |           |             |               |
|                           |         | DEVICE_NAME_MSB             | RW   | 0          | 16        | 0x0         | POR           |
| DEVICE_NAME_LSB           | 0x1E    |                             |      |            |           |             |               |
|                           |         | DEVICE_NAME_LSB             | RW   | 0          | 16        | 0x0         | POR           |
| NVM_1                     | 0x1F    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B11                  | RO   | 11         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B10                  | RO   | 10         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B9                   | RO   | 9          | 1         | 0x0         | POR           |
|                           |         | UNUSED_B8                   | RO   | 8          | 1         | 0x0         | POR           |
|                           |         | UNUSED_B7                   | RO   | 7          | 1         | 0x0         | POR           |
|                           |         | UNUSED_B6                   | RO   | 6          | 1         | 0x0         | POR           |
|                           |         | UNUSED_B5                   | RO   | 5          | 1         | 0x0         | POR           |
|                           |         | NVM_UPLOADS_COUNT           | RO   | 0          | 5         | 0x0         | POR           |
| NVM_2                     | 0x20    |                             |      |            |           |             |               |
|                           |         | NVM_WRITE_READ_CODE_CMD     | WO   | 0          | 16        | 0x0         | POR           |
| VCELL1                    | 0x21    |                             |      |            |           |             |               |
|                           |         | CRC_CFG_FAIL                | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | CRC_TRIM_CAL_FAIL           | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | GO2SHIP                     | WO   | 12         | 2         | 0x0         | POR           |
|                           |         | VCELL1_MEAS                 | RO   | 0          | 12        | 0x0         | POR           |
| VCELL2                    | 0x22    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | GO2STBY                     | WO   | 12         | 2         | 0x0         | POR           |
|                           |         | VCELL2_MEAS                 | RO   | 0          | 12        | 0x0         | POR           |
| VCELL3                    | 0x23    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | FUSE_TRIG_ARM               | WO   | 12         | 2         | 0x0         | POR           |
|                           |         | VCELL3_MEAS                 | RO   | 0          | 12        | 0x0         | POR           |
| VCELL4                    | 0x24    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | FUSE_TRIG_FIRE              | RW   | 12         | 2         | 0x1         | POR           |
|                           |         | VCELL4_MEAS                 | RO   | 0          | 12        | 0x0         | POR           |
| VCELL5                    | 0x25    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | VCELL5_MEAS                 | RO   | 0          | 12        | 0x0         | POR           |
| VCELLSUM                  | 0x26    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | VCELLSUM_MEAS               | RO   | 0          | 15        | 0x0         | POR           |
| VB                        | 0x27    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | VB_MEAS                     | RO   | 0          | 12        | 0x0         | POR           |
| NTC_GPIO                  | 0x28    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | NTC_MEAS                    | RO   | 0          | 12        | 0x0         | POR           |
| DIE_TEMP                  | 0x29    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | DIE_TEMP_MEAS               | RO   | 0          | 12        | 0x0         | POR           |
| DIAG_OV_OT_UT             | 0x2A    |                             |      |            |           |             |               |
|                           |         | DIE_OT                      | RLW  | 15         | 1         | 0x0         | POR           |
|                           |         | NTC_UT                      | RLW  | 14         | 1         | 0x0         | POR           |
|                           |         | NTC_SEVERE_OT               | RLW  | 13         | 1         | 0x0         | POR           |
|                           |         | NTC_OT                      | RLW  | 12         | 1         | 0x0         | POR           |
|                           |         | VB_SUM_CHECK_FAIL           | RLW  | 11         | 1         | 0x0         | POR           |
|                           |         | V_SEVERE_CELL5_OV           | RLW  | 10         | 1         | 0x0         | POR           |
|                           |         | V_SEVERE_CELL4_OV           | RLW  | 9          | 1         | 0x0         | POR           |
|                           |         | V_SEVERE_CELL3_OV           | RLW  | 8          | 1         | 0x0         | POR           |
|                           |         | V_SEVERE_CELL2_OV           | RLW  | 7          | 1         | 0x0         | POR           |
|                           |         | V_SEVERE_CELL1_OV           | RLW  | 6          | 1         | 0x0         | POR           |
|                           |         | VB_OV                       | RLW  | 5          | 1         | 0x0         | POR           |
|                           |         | CELL5_OV                    | RLW  | 4          | 1         | 0x0         | POR           |
|                           |         | CELL4_OV                    | RLW  | 3          | 1         | 0x0         | POR           |
|                           |         | CELL3_OV                    | RLW  | 2          | 1         | 0x0         | POR           |
|                           |         | CELL2_OV                    | RLW  | 1          | 1         | 0x0         | POR           |
|                           |         | CELL1_OV                    | RLW  | 0          | 1         | 0x0         | POR           |
| DIAG_UV                   | 0x2B    |                             |      |            |           |             |               |
|                           |         | V_SEVERE_CELL5_UV           | RLW  | 15         | 1         | 0x0         | POR           |
|                           |         | V_SEVERE_CELL4_UV           | RLW  | 14         | 1         | 0x0         | POR           |
|                           |         | V_SEVERE_CELL3_UV           | RLW  | 13         | 1         | 0x0         | POR           |
|                           |         | V_SEVERE_CELL2_UV           | RLW  | 12         | 1         | 0x0         | POR           |
|                           |         | V_SEVERE_CELL1_UV           | RLW  | 11         | 1         | 0x0         | POR           |
|                           |         | VB_UV                       | RLW  | 10         | 1         | 0x0         | POR           |
|                           |         | BAL5_UV                     | RLW  | 9          | 1         | 0x0         | POR           |
|                           |         | BAL4_UV                     | RLW  | 8          | 1         | 0x0         | POR           |
|                           |         | BAL3_UV                     | RLW  | 7          | 1         | 0x0         | POR           |
|                           |         | BAL2_UV                     | RLW  | 6          | 1         | 0x0         | POR           |
|                           |         | BAL1_UV                     | RLW  | 5          | 1         | 0x0         | POR           |
|                           |         | CELL5_UV                    | RLW  | 4          | 1         | 0x0         | POR           |
|                           |         | CELL4_UV                    | RLW  | 3          | 1         | 0x0         | POR           |
|                           |         | CELL3_UV                    | RLW  | 2          | 1         | 0x0         | POR           |
|                           |         | CELL2_UV                    | RLW  | 1          | 1         | 0x0         | POR           |
|                           |         | CELL1_UV                    | RLW  | 0          | 1         | 0x0         | POR           |
| CC_INST_MEAS              | 0x2C    |                             |      |            |           |             |               |
|                           |         | CC_CUR_INST_MEAS            | RO   | 0          | 16        | 0x0         | POR           |
| CC_ACC_MSB                | 0x2D    |                             |      |            |           |             |               |
|                           |         | CC_ACC_MSB_15               | RLW  | 15         | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_14               | RLW  | 14         | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_13               | RLW  | 13         | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_12               | RLW  | 12         | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_11               | RLW  | 11         | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_10               | RLW  | 10         | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_9                | RLW  | 9          | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_8                | RLW  | 8          | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_7                | RLW  | 7          | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_6                | RLW  | 6          | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_5                | RLW  | 5          | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_4                | RLW  | 4          | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_3                | RLW  | 3          | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_2                | RLW  | 2          | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_1                | RLW  | 1          | 1         | 0x0         | POR           |
|                           |         | CC_ACC_MSB_0                | RLW  | 0          | 1         | 0x0         | POR           |
| CC_ACC_LSB_CNTR           | 0x2E    |                             |      |            |           |             |               |
|                           |         | CC_ACC_LSB_7                | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | CC_ACC_LSB_6                | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | CC_ACC_LSB_5                | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | CC_ACC_LSB_4                | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | CC_ACC_LSB_3                | RO   | 11         | 1         | 0x0         | POR           |
|                           |         | CC_ACC_LSB_2                | RO   | 10         | 1         | 0x0         | POR           |
|                           |         | CC_ACC_LSB_1                | RO   | 9          | 1         | 0x0         | POR           |
|                           |         | CC_ACC_LSB_0                | RO   | 8          | 1         | 0x0         | POR           |
|                           |         | CC_SAMPLE_CNT_7             | RO   | 7          | 1         | 0x0         | POR           |
|                           |         | CC_SAMPLE_CNT_6             | RO   | 6          | 1         | 0x0         | POR           |
|                           |         | CC_SAMPLE_CNT_5             | RO   | 5          | 1         | 0x0         | POR           |
|                           |         | CC_SAMPLE_CNT_4             | RO   | 4          | 1         | 0x0         | POR           |
|                           |         | CC_SAMPLE_CNT_3             | RO   | 3          | 1         | 0x0         | POR           |
|                           |         | CC_SAMPLE_CNT_2             | RO   | 2          | 1         | 0x0         | POR           |
|                           |         | CC_SAMPLE_CNT_1             | RO   | 1          | 1         | 0x0         | POR           |
|                           |         | CC_SAMPLE_CNT_0             | RO   | 0          | 1         | 0x0         | POR           |
| DIAG_CURR                 | 0x2F    |                             |      |            |           |             |               |
|                           |         | UNUSED_B15                  | RO   | 15         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B14                  | RO   | 14         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B13                  | RO   | 13         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B12                  | RO   | 12         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B11                  | RO   | 11         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B10                  | RO   | 10         | 1         | 0x0         | POR           |
|                           |         | UNUSED_B9                   | RO   | 9          | 1         | 0x0         | POR           |
|                           |         | FAULTN_EXT                  | RLW  | 8          | 1         | 0x0         | POR           |
|                           |         | FUSE_EXT                    | RO   | 7          | 1         | 0x0         | POR           |
|                           |         | PERSIST_SC_DCHG             | RLW  | 6          | 1         | 0x0         | POR           |
|                           |         | SC_DCHG                     | RLW  | 5          | 1         | 0x0         | POR           |
|                           |         | PERSIST_OVC_DCHG            | RLW  | 4          | 1         | 0x0         | POR           |
|                           |         | PERSIST_OVC_CHG             | RLW  | 3          | 1         | 0x0         | POR           |
|                           |         | OVC_DCHG                    | RLW  | 2          | 1         | 0x0         | POR           |
|                           |         | OVC_CHG                     | RLW  | 1          | 1         | 0x0         | POR           |
|                           |         | CC_SAT                      | RO   | 0          | 1         | 0x0         | POR           |
