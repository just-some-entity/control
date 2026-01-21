import React from "react";

import { ControlCard } from "@/control/ControlCard";
import { Page } from "@/components/Page";
import { ControlGrid } from "@/control/ControlGrid";
import { TimeSeriesValueNumeric } from "@/control/TimeSeriesValue";
import { MinMaxValue, TIMEFRAME_OPTIONS } from "@/control/MinMaxValue";
import { EditValue } from "@/control/EditValue";
import { Label } from "@/control/Label";

import { useLaser1 as usePellet1 } from "./usePellet";

import { SelectionGroupBoolean } from "@/control/SelectionGroup";
import { createTimeSeries, TimeSeries } from "@/lib/timeseries";
import { roundToDecimals } from "@/lib/decimal";
import { Badge } from "@/components/ui/badge";

export function Pellet1ControlPage() {
    const {
        diameter,
        x_diameter,
        y_diameter,
        roundness,
        state,
        defaultState,
        setTargetDiameter,
        setLowerTolerance,
        setHigherTolerance,
    } = usePellet1();

    // Extract values from consolidated state
    const targetDiameter = state?.laser_state?.target_diameter ?? 0;
    const lowerTolerance = state?.laser_state?.lower_tolerance ?? 0;
    const higherTolerance = state?.laser_state?.higher_tolerance ?? 0;

    // Detect if this is a 2-axis laser
    const isTwoAxis = !!x_diameter?.current || !!y_diameter?.current;
    // Shared timeframe state (default 5 minutes)
    const [timeframe, setTimeframe] = React.useState<number>(5 * 60 * 1000);

    return (
        <Page>
            <ControlGrid columns={2}>
                <ControlCard title="Inverter Configuration">
                    <Label label="Rotation Direction">
                        <SelectionGroupBoolean
                            value={true}
                            optionTrue={{ children: "Forward" }}
                            optionFalse={{ children: "Backward" }}
                            onChange={(v) => {}}
                        />
                    </Label>

                    <Label label="Set Target Frequency">
                        <EditValue
                            title="Set Target Frequency"
                            value={0}
                            unit="Hz"
                            step={1}
                            min={0}
                            max={99}
                            renderValue={(value) => value.toFixed(0)}
                            onChange={(val) => {
                                // setTargetDiameter(val);
                            }}
                            defaultValue={5}
                        />
                    </Label>
                    <Label label="Set Acceleration Level">
                        <EditValue
                            title="Set Acceleration Level"
                            value={1}
                            unit={undefined}
                            step={1}
                            min={1}
                            max={15}
                            renderValue={(value) => value.toFixed(0)}
                            onChange={(val) => setLowerTolerance(val)}
                            defaultValue={7}
                        />
                    </Label>
                    <Label label="Set Deceleration Level">
                        <EditValue
                            title="Set Deceleration Level"
                            value={1}
                            unit={undefined}
                            step={1}
                            min={1}
                            max={15}
                            renderValue={(value) => value.toFixed(0)}
                            onChange={(val) => setLowerTolerance(val)}
                            defaultValue={7}
                        />
                    </Label>
                </ControlCard>

                <ControlCard title="Inverter Status">
                    <Badge
                        className={`text-md ${true ? "bg-white-600 border-green-600 text-green-600" : "bg-red-600 font-bold text-white"} mx-auto h-12 w-[100%] border-3 text-lg`}
                    >
                        {true ? "Running" : "PULSE OVERCURRENT"}
                    </Badge>

                    <TimeSeriesValueNumeric
                        label="Frequency"
                        unit="Hz"
                        renderValue={(value) => roundToDecimals(value, 1)}
                        timeseries={createTimeSeries().initialTimeSeries}
                    />

                    <TimeSeriesValueNumeric
                        label="Temperature"
                        unit="C"
                        renderValue={(value) => roundToDecimals(value, 1)}
                        timeseries={createTimeSeries().initialTimeSeries}
                    />

                    <TimeSeriesValueNumeric
                        label="Voltage"
                        unit="V"
                        renderValue={(value) => roundToDecimals(value, 1)}
                        timeseries={createTimeSeries().initialTimeSeries}
                    />

                    <TimeSeriesValueNumeric
                        label="Current"
                        unit="A"
                        renderValue={(value) => roundToDecimals(value, 1)}
                        timeseries={createTimeSeries().initialTimeSeries}
                    />
                </ControlCard>
            </ControlGrid>
        </Page>
    );
}
